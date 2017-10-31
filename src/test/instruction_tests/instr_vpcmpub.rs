use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpcmpub_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUB, operand1: Some(Direct(K7)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM7)), operand4: Some(Literal8(108)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 93, 12, 62, 255, 108], OperandSize::Dword)
}

#[test]
fn vpcmpub_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUB, operand1: Some(Direct(K3)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(ECX, 535623168, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(88)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 117, 15, 62, 153, 0, 246, 236, 31, 88], OperandSize::Dword)
}

#[test]
fn vpcmpub_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUB, operand1: Some(Direct(K6)), operand2: Some(Direct(XMM11)), operand3: Some(Direct(XMM23)), operand4: Some(Literal8(23)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 179, 37, 10, 62, 247, 23], OperandSize::Qword)
}

#[test]
fn vpcmpub_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUB, operand1: Some(Direct(K3)), operand2: Some(Direct(XMM13)), operand3: Some(IndirectScaledIndexed(RDI, RSI, Eight, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(109)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 21, 10, 62, 28, 247, 109], OperandSize::Qword)
}

#[test]
fn vpcmpub_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUB, operand1: Some(Direct(K5)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM4)), operand4: Some(Literal8(64)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 77, 46, 62, 236, 64], OperandSize::Dword)
}

#[test]
fn vpcmpub_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUB, operand1: Some(Direct(K7)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledDisplaced(EDI, Four, 229660851, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(115)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 117, 42, 62, 60, 189, 179, 88, 176, 13, 115], OperandSize::Dword)
}

#[test]
fn vpcmpub_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUB, operand1: Some(Direct(K5)), operand2: Some(Direct(YMM9)), operand3: Some(Direct(YMM28)), operand4: Some(Literal8(33)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 147, 53, 46, 62, 236, 33], OperandSize::Qword)
}

#[test]
fn vpcmpub_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUB, operand1: Some(Direct(K7)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexed(RBX, RAX, Four, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(77)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 93, 47, 62, 60, 131, 77], OperandSize::Qword)
}

#[test]
fn vpcmpub_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUB, operand1: Some(Direct(K3)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM1)), operand4: Some(Literal8(90)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 77, 77, 62, 217, 90], OperandSize::Dword)
}

#[test]
fn vpcmpub_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUB, operand1: Some(Direct(K1)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledDisplaced(EDX, Eight, 888402496, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(3)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 93, 79, 62, 12, 213, 64, 242, 243, 52, 3], OperandSize::Dword)
}

#[test]
fn vpcmpub_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUB, operand1: Some(Direct(K4)), operand2: Some(Direct(ZMM8)), operand3: Some(Direct(ZMM15)), operand4: Some(Literal8(2)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 211, 61, 74, 62, 231, 2], OperandSize::Qword)
}

#[test]
fn vpcmpub_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUB, operand1: Some(Direct(K4)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectDisplaced(RAX, 227670131, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(17)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 77, 76, 62, 160, 115, 248, 145, 13, 17], OperandSize::Qword)
}

