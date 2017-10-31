use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpblendw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM4)), operand4: Some(Literal8(30)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 14, 212, 30], OperandSize::Dword)
}

#[test]
fn vpblendw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectDisplaced(EAX, 767449382, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(124)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 73, 14, 160, 38, 89, 190, 45, 124], OperandSize::Dword)
}

#[test]
fn vpblendw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM4)), operand4: Some(Literal8(40)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 97, 14, 196, 40], OperandSize::Qword)
}

#[test]
fn vpblendw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledDisplaced(RAX, Four, 2075869786, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(54)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 73, 14, 28, 133, 90, 66, 187, 123, 54], OperandSize::Qword)
}

#[test]
fn vpblendw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM7)), operand4: Some(Literal8(106)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 117, 14, 207, 106], OperandSize::Dword)
}

#[test]
fn vpblendw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledDisplaced(EDX, Eight, 1866843705, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(78)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 85, 14, 20, 213, 57, 198, 69, 111, 78], OperandSize::Dword)
}

#[test]
fn vpblendw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM7)), operand4: Some(Literal8(8)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 85, 14, 199, 8], OperandSize::Qword)
}

#[test]
fn vpblendw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RBX, Eight, 1155956565, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(28)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 109, 14, 164, 222, 85, 127, 230, 68, 28], OperandSize::Qword)
}

