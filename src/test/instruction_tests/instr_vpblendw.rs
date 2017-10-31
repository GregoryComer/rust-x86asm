use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpblendw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM0)), operand4: Some(Literal8(105)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 97, 14, 232, 105], OperandSize::Dword)
}

#[test]
fn vpblendw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(37)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 113, 14, 62, 37], OperandSize::Dword)
}

#[test]
fn vpblendw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM0)), operand4: Some(Literal8(56)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 65, 14, 200, 56], OperandSize::Qword)
}

#[test]
fn vpblendw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexed(RSI, RAX, Two, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(38)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 105, 14, 20, 70, 38], OperandSize::Qword)
}

#[test]
fn vpblendw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDW, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM3)), operand4: Some(Literal8(41)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 69, 14, 251, 41], OperandSize::Dword)
}

#[test]
fn vpblendw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledDisplaced(EAX, Eight, 1009015523, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(87)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 14, 44, 197, 227, 90, 36, 60, 87], OperandSize::Dword)
}

#[test]
fn vpblendw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM0)), operand4: Some(Literal8(97)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 77, 14, 232, 97], OperandSize::Qword)
}

#[test]
fn vpblendw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledDisplaced(RDI, Two, 538717656, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(60)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 85, 14, 52, 125, 216, 45, 28, 32, 60], OperandSize::Qword)
}

