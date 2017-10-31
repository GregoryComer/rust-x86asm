use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn paddw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDW, operand1: Some(Direct(MM7)), operand2: Some(Direct(MM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 253, 248], OperandSize::Dword)
}

#[test]
fn paddw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDW, operand1: Some(Direct(MM2)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, EDX, Two, 1309937325, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 253, 148, 87, 173, 14, 20, 78], OperandSize::Dword)
}

#[test]
fn paddw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDW, operand1: Some(Direct(MM5)), operand2: Some(Direct(MM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 253, 238], OperandSize::Qword)
}

#[test]
fn paddw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDW, operand1: Some(Direct(MM3)), operand2: Some(Indirect(RDX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 253, 26], OperandSize::Qword)
}

#[test]
fn paddw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 253, 240], OperandSize::Dword)
}

#[test]
fn paddw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDW, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexed(ECX, ECX, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 253, 44, 137], OperandSize::Dword)
}

#[test]
fn paddw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 253, 246], OperandSize::Qword)
}

#[test]
fn paddw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDW, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RAX, Eight, 1018806878, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 253, 164, 192, 94, 194, 185, 60], OperandSize::Qword)
}

