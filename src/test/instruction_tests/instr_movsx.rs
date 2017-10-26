use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn movsx_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSX, operand1: Some(Direct(SP)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 190, 225], OperandSize::Word)
}

#[test]
fn movsx_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSX, operand1: Some(Direct(CX)), operand2: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 7398, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 190, 136, 230, 28], OperandSize::Word)
}

#[test]
fn movsx_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSX, operand1: Some(Direct(DX)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 190, 209], OperandSize::Dword)
}

#[test]
fn movsx_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSX, operand1: Some(Direct(SI)), operand2: Some(Indirect(EAX, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 190, 48], OperandSize::Dword)
}

#[test]
fn movsx_5() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSX, operand1: Some(Direct(SI)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 190, 242], OperandSize::Qword)
}

#[test]
fn movsx_6() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSX, operand1: Some(Direct(DI)), operand2: Some(Indirect(RAX, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 190, 56], OperandSize::Qword)
}

#[test]
fn movsx_7() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSX, operand1: Some(Direct(ECX)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 190, 201], OperandSize::Word)
}

#[test]
fn movsx_8() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSX, operand1: Some(Direct(ESI)), operand2: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 28904, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 190, 179, 232, 112], OperandSize::Word)
}

#[test]
fn movsx_9() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSX, operand1: Some(Direct(ESI)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 190, 241], OperandSize::Dword)
}

#[test]
fn movsx_10() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSX, operand1: Some(Direct(ECX)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, ECX, Two, 1863240117, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 190, 140, 78, 181, 201, 14, 111], OperandSize::Dword)
}

#[test]
fn movsx_11() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSX, operand1: Some(Direct(ESI)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 190, 243], OperandSize::Qword)
}

#[test]
fn movsx_12() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSX, operand1: Some(Direct(EDX)), operand2: Some(Indirect(RBX, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 190, 19], OperandSize::Qword)
}

#[test]
fn movsx_13() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSX, operand1: Some(Direct(RBP)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 190, 235], OperandSize::Qword)
}

#[test]
fn movsx_14() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSX, operand1: Some(Direct(RDX)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RAX, Eight, 1096312930, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 190, 148, 195, 98, 104, 88, 65], OperandSize::Qword)
}

#[test]
fn movsx_15() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSX, operand1: Some(Direct(EBP)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 191, 237], OperandSize::Word)
}

#[test]
fn movsx_16() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSX, operand1: Some(Direct(ESI)), operand2: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 108, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 191, 114, 108], OperandSize::Word)
}

#[test]
fn movsx_17() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSX, operand1: Some(Direct(EDI)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 191, 250], OperandSize::Dword)
}

#[test]
fn movsx_18() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSX, operand1: Some(Direct(ECX)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, EBX, Two, 434427120, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 191, 140, 88, 240, 212, 228, 25], OperandSize::Dword)
}

#[test]
fn movsx_19() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSX, operand1: Some(Direct(ESP)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 191, 225], OperandSize::Qword)
}

#[test]
fn movsx_20() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSX, operand1: Some(Direct(EBP)), operand2: Some(Indirect(RCX, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 191, 41], OperandSize::Qword)
}

#[test]
fn movsx_21() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSX, operand1: Some(Direct(RDI)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 191, 254], OperandSize::Qword)
}

#[test]
fn movsx_22() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSX, operand1: Some(Direct(RBP)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RBX, Eight, 2092965134, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 191, 172, 222, 14, 29, 192, 124], OperandSize::Qword)
}

