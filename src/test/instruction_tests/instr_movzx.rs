use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn movzx_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVZX, operand1: Some(Direct(BX)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 182, 218], OperandSize::Word)
}

#[test]
fn movzx_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVZX, operand1: Some(Direct(SI)), operand2: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 19469, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 182, 177, 13, 76], OperandSize::Word)
}

#[test]
fn movzx_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVZX, operand1: Some(Direct(BX)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 182, 219], OperandSize::Dword)
}

#[test]
fn movzx_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVZX, operand1: Some(Direct(DX)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, EAX, Two, 1996889227, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 182, 148, 66, 139, 28, 6, 119], OperandSize::Dword)
}

#[test]
fn movzx_5() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVZX, operand1: Some(Direct(BX)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 182, 219], OperandSize::Qword)
}

#[test]
fn movzx_6() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVZX, operand1: Some(Direct(DI)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RDI, Four, 1093027425, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 182, 188, 190, 97, 70, 38, 65], OperandSize::Qword)
}

#[test]
fn movzx_7() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVZX, operand1: Some(Direct(EDI)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 182, 250], OperandSize::Word)
}

#[test]
fn movzx_8() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVZX, operand1: Some(Direct(EDI)), operand2: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 29460, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 182, 186, 20, 115], OperandSize::Word)
}

#[test]
fn movzx_9() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVZX, operand1: Some(Direct(EDI)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 182, 249], OperandSize::Dword)
}

#[test]
fn movzx_10() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVZX, operand1: Some(Direct(EBX)), operand2: Some(IndirectScaledIndexed(EDI, ECX, Eight, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 182, 28, 207], OperandSize::Dword)
}

#[test]
fn movzx_11() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVZX, operand1: Some(Direct(ESP)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 182, 227], OperandSize::Qword)
}

#[test]
fn movzx_12() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVZX, operand1: Some(Direct(EBX)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RDI, Eight, 1625458252, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 182, 156, 250, 76, 134, 226, 96], OperandSize::Qword)
}

#[test]
fn movzx_13() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVZX, operand1: Some(Direct(RBP)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 182, 234], OperandSize::Qword)
}

#[test]
fn movzx_14() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVZX, operand1: Some(Direct(RSI)), operand2: Some(Indirect(RDX, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 182, 50], OperandSize::Qword)
}

#[test]
fn movzx_15() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVZX, operand1: Some(Direct(ECX)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 183, 207], OperandSize::Word)
}

#[test]
fn movzx_16() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVZX, operand1: Some(Direct(ESP)), operand2: Some(IndirectScaledIndexed(BP, DI, One, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 183, 35], OperandSize::Word)
}

#[test]
fn movzx_17() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVZX, operand1: Some(Direct(ECX)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 183, 204], OperandSize::Dword)
}

#[test]
fn movzx_18() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVZX, operand1: Some(Direct(EBX)), operand2: Some(IndirectScaledIndexed(EDX, EBX, Two, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 183, 28, 90], OperandSize::Dword)
}

#[test]
fn movzx_19() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVZX, operand1: Some(Direct(ECX)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 183, 202], OperandSize::Qword)
}

#[test]
fn movzx_20() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVZX, operand1: Some(Direct(ECX)), operand2: Some(Indirect(RSI, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 183, 14], OperandSize::Qword)
}

#[test]
fn movzx_21() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVZX, operand1: Some(Direct(RSP)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 183, 229], OperandSize::Qword)
}

#[test]
fn movzx_22() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVZX, operand1: Some(Direct(RCX)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RDX, Four, 367323960, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 183, 140, 144, 56, 235, 228, 21], OperandSize::Qword)
}

