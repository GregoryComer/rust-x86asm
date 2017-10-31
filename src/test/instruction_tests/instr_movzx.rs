use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn movzx_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVZX, operand1: Some(Direct(SI)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 182, 243], OperandSize::Word)
}

#[test]
fn movzx_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVZX, operand1: Some(Direct(BP)), operand2: Some(Indirect(DI, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 182, 45], OperandSize::Word)
}

#[test]
fn movzx_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVZX, operand1: Some(Direct(SI)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 182, 241], OperandSize::Dword)
}

#[test]
fn movzx_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVZX, operand1: Some(Direct(CX)), operand2: Some(IndirectScaledIndexed(EBX, ECX, Two, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 182, 12, 75], OperandSize::Dword)
}

#[test]
fn movzx_5() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVZX, operand1: Some(Direct(SP)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 182, 225], OperandSize::Qword)
}

#[test]
fn movzx_6() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVZX, operand1: Some(Direct(SI)), operand2: Some(IndirectScaledDisplaced(RSI, Two, 929698740, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 182, 52, 117, 180, 19, 106, 55], OperandSize::Qword)
}

#[test]
fn movzx_7() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVZX, operand1: Some(Direct(EDX)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 182, 210], OperandSize::Word)
}

#[test]
fn movzx_8() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVZX, operand1: Some(Direct(EDX)), operand2: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 10992, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 182, 144, 240, 42], OperandSize::Word)
}

#[test]
fn movzx_9() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVZX, operand1: Some(Direct(ESI)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 182, 243], OperandSize::Dword)
}

#[test]
fn movzx_10() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVZX, operand1: Some(Direct(ESI)), operand2: Some(IndirectScaledDisplaced(ECX, Eight, 1360801274, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 182, 52, 205, 250, 45, 28, 81], OperandSize::Dword)
}

#[test]
fn movzx_11() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVZX, operand1: Some(Direct(EBP)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 182, 233], OperandSize::Qword)
}

#[test]
fn movzx_12() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVZX, operand1: Some(Direct(EBP)), operand2: Some(Indirect(RSI, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 182, 46], OperandSize::Qword)
}

#[test]
fn movzx_13() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVZX, operand1: Some(Direct(RCX)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 182, 201], OperandSize::Qword)
}

#[test]
fn movzx_14() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVZX, operand1: Some(Direct(RDX)), operand2: Some(IndirectScaledDisplaced(RDX, Four, 63553633, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 182, 20, 149, 97, 192, 201, 3], OperandSize::Qword)
}

#[test]
fn movzx_15() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVZX, operand1: Some(Direct(EBP)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 183, 239], OperandSize::Word)
}

#[test]
fn movzx_16() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVZX, operand1: Some(Direct(EBP)), operand2: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 28199, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 183, 171, 39, 110], OperandSize::Word)
}

#[test]
fn movzx_17() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVZX, operand1: Some(Direct(EBX)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 183, 219], OperandSize::Dword)
}

#[test]
fn movzx_18() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVZX, operand1: Some(Direct(EDI)), operand2: Some(IndirectScaledDisplaced(EBX, Eight, 1640916185, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 183, 60, 221, 217, 100, 206, 97], OperandSize::Dword)
}

#[test]
fn movzx_19() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVZX, operand1: Some(Direct(EBX)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 183, 221], OperandSize::Qword)
}

#[test]
fn movzx_20() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVZX, operand1: Some(Direct(EBP)), operand2: Some(IndirectDisplaced(RAX, 215227335, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 183, 168, 199, 27, 212, 12], OperandSize::Qword)
}

#[test]
fn movzx_21() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVZX, operand1: Some(Direct(RBX)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 183, 223], OperandSize::Qword)
}

#[test]
fn movzx_22() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVZX, operand1: Some(Direct(RDI)), operand2: Some(Indirect(RCX, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 183, 57], OperandSize::Qword)
}

