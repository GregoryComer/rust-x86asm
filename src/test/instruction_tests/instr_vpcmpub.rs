use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpcmpub_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUB, operand1: Some(Direct(K4)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM7)), operand4: Some(Literal8(101)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 109, 15, 62, 231, 101], OperandSize::Dword)
}

#[test]
fn vpcmpub_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUB, operand1: Some(Direct(K7)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexed(EDI, EDX, Eight, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(69)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 109, 11, 62, 60, 215, 69], OperandSize::Dword)
}

#[test]
fn vpcmpub_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUB, operand1: Some(Direct(K4)), operand2: Some(Direct(XMM28)), operand3: Some(Direct(XMM31)), operand4: Some(Literal8(100)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 147, 29, 7, 62, 231, 100], OperandSize::Qword)
}

#[test]
fn vpcmpub_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUB, operand1: Some(Direct(K7)), operand2: Some(Direct(XMM23)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RBX, Eight, 503841695, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(1)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 69, 4, 62, 188, 223, 159, 3, 8, 30, 1], OperandSize::Qword)
}

#[test]
fn vpcmpub_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUB, operand1: Some(Direct(K1)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM3)), operand4: Some(Literal8(88)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 101, 45, 62, 203, 88], OperandSize::Dword)
}

#[test]
fn vpcmpub_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUB, operand1: Some(Direct(K2)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectDisplaced(ESI, 291499751, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(104)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 85, 46, 62, 150, 231, 238, 95, 17, 104], OperandSize::Dword)
}

#[test]
fn vpcmpub_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUB, operand1: Some(Direct(K4)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM7)), operand4: Some(Literal8(40)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 109, 44, 62, 231, 40], OperandSize::Qword)
}

#[test]
fn vpcmpub_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUB, operand1: Some(Direct(K2)), operand2: Some(Direct(YMM20)), operand3: Some(IndirectScaledDisplaced(RDX, Eight, 930646955, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(31)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 93, 38, 62, 20, 213, 171, 139, 120, 55, 31], OperandSize::Qword)
}

#[test]
fn vpcmpub_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUB, operand1: Some(Direct(K7)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM6)), operand4: Some(Literal8(68)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 93, 78, 62, 254, 68], OperandSize::Dword)
}

#[test]
fn vpcmpub_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUB, operand1: Some(Direct(K6)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledDisplaced(EDX, Two, 422345962, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(104)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 93, 76, 62, 52, 85, 234, 124, 44, 25, 104], OperandSize::Dword)
}

#[test]
fn vpcmpub_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUB, operand1: Some(Direct(K1)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM7)), operand4: Some(Literal8(15)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 69, 78, 62, 207, 15], OperandSize::Qword)
}

#[test]
fn vpcmpub_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUB, operand1: Some(Direct(K5)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledDisplaced(RDI, Eight, 601121325, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(108)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 101, 74, 62, 44, 253, 45, 98, 212, 35, 108], OperandSize::Qword)
}

