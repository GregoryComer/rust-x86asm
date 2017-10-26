use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn punpckhwd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHWD, operand1: Some(Direct(MM2)), operand2: Some(Direct(MM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 105, 209], OperandSize::Dword)
}

#[test]
fn punpckhwd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHWD, operand1: Some(Direct(MM1)), operand2: Some(IndirectScaledDisplaced(ECX, Four, 312289583, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 105, 12, 141, 47, 41, 157, 18], OperandSize::Dword)
}

#[test]
fn punpckhwd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHWD, operand1: Some(Direct(MM4)), operand2: Some(Direct(MM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 105, 231], OperandSize::Qword)
}

#[test]
fn punpckhwd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHWD, operand1: Some(Direct(MM0)), operand2: Some(IndirectScaledDisplaced(RAX, Eight, 2086364291, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 105, 4, 197, 131, 100, 91, 124], OperandSize::Qword)
}

#[test]
fn punpckhwd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHWD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 105, 208], OperandSize::Dword)
}

#[test]
fn punpckhwd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHWD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(EDX, Eight, 1471554944, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 105, 36, 213, 128, 37, 182, 87], OperandSize::Dword)
}

#[test]
fn punpckhwd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHWD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 105, 237], OperandSize::Qword)
}

#[test]
fn punpckhwd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKHWD, operand1: Some(Direct(XMM5)), operand2: Some(Indirect(RCX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 105, 41], OperandSize::Qword)
}

