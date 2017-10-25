use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn psignw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PSIGNW, operand1: Some(Direct(MM1)), operand2: Some(Direct(MM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 9, 201], OperandSize::Dword)
}

#[test]
fn psignw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PSIGNW, operand1: Some(Direct(MM5)), operand2: Some(Indirect(ESI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 9, 46], OperandSize::Dword)
}

#[test]
fn psignw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PSIGNW, operand1: Some(Direct(MM0)), operand2: Some(Direct(MM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 9, 194], OperandSize::Qword)
}

#[test]
fn psignw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PSIGNW, operand1: Some(Direct(MM3)), operand2: Some(IndirectScaledDisplaced(RDX, Four, 1478052480, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 9, 28, 149, 128, 74, 25, 88], OperandSize::Qword)
}

#[test]
fn psignw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PSIGNW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 9, 199], OperandSize::Dword)
}

#[test]
fn psignw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PSIGNW, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, EAX, Four, 193869362, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 9, 172, 129, 50, 54, 142, 11], OperandSize::Dword)
}

#[test]
fn psignw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PSIGNW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 9, 208], OperandSize::Qword)
}

#[test]
fn psignw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PSIGNW, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RAX, Two, 1107448286, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 9, 172, 71, 222, 81, 2, 66], OperandSize::Qword)
}

