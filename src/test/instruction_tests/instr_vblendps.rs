use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vblendps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM0)), operand4: Some(Literal8(11)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 97, 12, 224, 11], OperandSize::Dword)
}

#[test]
fn vblendps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(EDI, ECX, Eight, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(27)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 81, 12, 36, 207, 27], OperandSize::Dword)
}

#[test]
fn vblendps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM5)), operand4: Some(Literal8(65)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 73, 12, 245, 65], OperandSize::Qword)
}

#[test]
fn vblendps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RAX, Eight, 701779519, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(60)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 105, 12, 172, 195, 63, 78, 212, 41, 60], OperandSize::Qword)
}

#[test]
fn vblendps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDPS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM7)), operand4: Some(Literal8(56)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 12, 247, 56], OperandSize::Dword)
}

#[test]
fn vblendps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM5)), operand3: Some(Indirect(EDI, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(68)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 85, 12, 23, 68], OperandSize::Dword)
}

#[test]
fn vblendps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDPS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM7)), operand4: Some(Literal8(126)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 77, 12, 239, 126], OperandSize::Qword)
}

#[test]
fn vblendps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDPS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledDisplaced(RDX, Eight, 354653808, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(37)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 12, 60, 213, 112, 150, 35, 21, 37], OperandSize::Qword)
}

