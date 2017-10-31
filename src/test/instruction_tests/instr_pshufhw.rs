use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn pshufhw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PSHUFHW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(Literal8(25)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 112, 248, 25], OperandSize::Dword)
}

#[test]
fn pshufhw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PSHUFHW, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledDisplaced(EAX, Two, 1833020502, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(65)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 112, 60, 69, 86, 172, 65, 109, 65], OperandSize::Dword)
}

#[test]
fn pshufhw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PSHUFHW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(Literal8(14)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 112, 219, 14], OperandSize::Qword)
}

#[test]
fn pshufhw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PSHUFHW, operand1: Some(Direct(XMM7)), operand2: Some(IndirectDisplaced(RDX, 614693016, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(79)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 112, 186, 152, 120, 163, 36, 79], OperandSize::Qword)
}

