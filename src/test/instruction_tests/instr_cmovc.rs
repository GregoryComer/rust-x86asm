use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmovc_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVC, operand1: Some(Direct(DI)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 66, 254], OperandSize::Word)
}

#[test]
fn cmovc_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVC, operand1: Some(Direct(DX)), operand2: Some(Memory(26985, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 66, 22, 105, 105], OperandSize::Word)
}

#[test]
fn cmovc_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVC, operand1: Some(Direct(BP)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 66, 239], OperandSize::Dword)
}

#[test]
fn cmovc_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVC, operand1: Some(Direct(BP)), operand2: Some(IndirectScaledIndexed(EBX, ECX, Two, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 66, 44, 75], OperandSize::Dword)
}

#[test]
fn cmovc_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVC, operand1: Some(Direct(DX)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 66, 212], OperandSize::Qword)
}

#[test]
fn cmovc_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVC, operand1: Some(Direct(BP)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RAX, Two, 1323225372, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 66, 172, 64, 28, 209, 222, 78], OperandSize::Qword)
}

#[test]
fn cmovc_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVC, operand1: Some(Direct(EBX)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 66, 222], OperandSize::Word)
}

#[test]
fn cmovc_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVC, operand1: Some(Direct(EDI)), operand2: Some(IndirectDisplaced(BX, 3748, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 66, 191, 164, 14], OperandSize::Word)
}

#[test]
fn cmovc_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVC, operand1: Some(Direct(EBP)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 66, 235], OperandSize::Dword)
}

#[test]
fn cmovc_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVC, operand1: Some(Direct(ESI)), operand2: Some(IndirectScaledDisplaced(ECX, Four, 1973367833, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 66, 52, 141, 25, 52, 159, 117], OperandSize::Dword)
}

#[test]
fn cmovc_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVC, operand1: Some(Direct(ESP)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 66, 225], OperandSize::Qword)
}

#[test]
fn cmovc_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVC, operand1: Some(Direct(EDI)), operand2: Some(IndirectScaledDisplaced(RDX, Two, 1460926283, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 66, 60, 85, 75, 247, 19, 87], OperandSize::Qword)
}

#[test]
fn cmovc_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVC, operand1: Some(Direct(RDI)), operand2: Some(Direct(RDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 66, 250], OperandSize::Qword)
}

#[test]
fn cmovc_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVC, operand1: Some(Direct(RSI)), operand2: Some(Indirect(RCX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 66, 49], OperandSize::Qword)
}

