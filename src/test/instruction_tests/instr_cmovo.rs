use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmovo_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVO, operand1: Some(Direct(CX)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 64, 206], OperandSize::Word)
}

#[test]
fn cmovo_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVO, operand1: Some(Direct(DX)), operand2: Some(IndirectDisplaced(BP, 172, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 64, 150, 172, 0], OperandSize::Word)
}

#[test]
fn cmovo_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVO, operand1: Some(Direct(BP)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 64, 238], OperandSize::Dword)
}

#[test]
fn cmovo_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVO, operand1: Some(Direct(DX)), operand2: Some(IndirectScaledDisplaced(EAX, Four, 750933338, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 64, 20, 133, 90, 85, 194, 44], OperandSize::Dword)
}

#[test]
fn cmovo_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVO, operand1: Some(Direct(BX)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 64, 220], OperandSize::Qword)
}

#[test]
fn cmovo_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVO, operand1: Some(Direct(DI)), operand2: Some(IndirectDisplaced(RAX, 1945086434, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 64, 184, 226, 169, 239, 115], OperandSize::Qword)
}

#[test]
fn cmovo_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVO, operand1: Some(Direct(EBX)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 64, 219], OperandSize::Word)
}

#[test]
fn cmovo_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVO, operand1: Some(Direct(EBX)), operand2: Some(Indirect(BX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 64, 31], OperandSize::Word)
}

#[test]
fn cmovo_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVO, operand1: Some(Direct(ECX)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 64, 206], OperandSize::Dword)
}

#[test]
fn cmovo_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVO, operand1: Some(Direct(EDI)), operand2: Some(Indirect(EAX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 64, 56], OperandSize::Dword)
}

#[test]
fn cmovo_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVO, operand1: Some(Direct(ECX)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 64, 207], OperandSize::Qword)
}

#[test]
fn cmovo_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVO, operand1: Some(Direct(EBP)), operand2: Some(IndirectScaledDisplaced(RBX, Two, 162410483, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 64, 44, 93, 243, 47, 174, 9], OperandSize::Qword)
}

#[test]
fn cmovo_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVO, operand1: Some(Direct(RDX)), operand2: Some(Direct(RDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 64, 210], OperandSize::Qword)
}

#[test]
fn cmovo_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVO, operand1: Some(Direct(RBX)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RDI, Eight, 1732911895, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 64, 156, 254, 23, 35, 74, 103], OperandSize::Qword)
}

