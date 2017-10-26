use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpcmpeqb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 116, 197], OperandSize::Dword)
}

#[test]
fn vpcmpeqb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQB, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectDisplaced(ESI, 218569249, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 116, 190, 33, 26, 7, 13], OperandSize::Dword)
}

#[test]
fn vpcmpeqb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQB, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 116, 228], OperandSize::Qword)
}

#[test]
fn vpcmpeqb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQB, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexed(RSI, RDX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 116, 28, 150], OperandSize::Qword)
}

#[test]
fn vpcmpeqb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQB, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 116, 210], OperandSize::Dword)
}

#[test]
fn vpcmpeqb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQB, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM2)), operand3: Some(Indirect(EAX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 116, 56], OperandSize::Dword)
}

#[test]
fn vpcmpeqb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQB, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 116, 210], OperandSize::Qword)
}

#[test]
fn vpcmpeqb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQB, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexed(RDI, RDX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 116, 44, 87], OperandSize::Qword)
}

#[test]
fn vpcmpeqb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQB, operand1: Some(Direct(K2)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 93, 14, 116, 215], OperandSize::Dword)
}

#[test]
fn vpcmpeqb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQB, operand1: Some(Direct(K2)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, ESI, Two, 693182447, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 117, 11, 116, 148, 114, 239, 31, 81, 41], OperandSize::Dword)
}

#[test]
fn vpcmpeqb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQB, operand1: Some(Direct(K6)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM21)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 177, 101, 12, 116, 245], OperandSize::Qword)
}

#[test]
fn vpcmpeqb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQB, operand1: Some(Direct(K3)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexed(RBX, RDI, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 101, 12, 116, 28, 187], OperandSize::Qword)
}

#[test]
fn vpcmpeqb_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQB, operand1: Some(Direct(K2)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 109, 47, 116, 211], OperandSize::Dword)
}

#[test]
fn vpcmpeqb_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQB, operand1: Some(Direct(K4)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledDisplaced(EDX, Eight, 1045312573, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 125, 44, 116, 36, 213, 61, 52, 78, 62], OperandSize::Dword)
}

#[test]
fn vpcmpeqb_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQB, operand1: Some(Direct(K7)), operand2: Some(Direct(YMM30)), operand3: Some(Direct(YMM20)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 177, 13, 34, 116, 252], OperandSize::Qword)
}

#[test]
fn vpcmpeqb_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQB, operand1: Some(Direct(K3)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RDX, Four, 908091616, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 77, 46, 116, 156, 151, 224, 96, 32, 54], OperandSize::Qword)
}

#[test]
fn vpcmpeqb_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQB, operand1: Some(Direct(K4)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 117, 76, 116, 229], OperandSize::Dword)
}

#[test]
fn vpcmpeqb_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQB, operand1: Some(Direct(K2)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectDisplaced(ECX, 1677560636, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 69, 76, 116, 145, 60, 139, 253, 99], OperandSize::Dword)
}

#[test]
fn vpcmpeqb_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQB, operand1: Some(Direct(K4)), operand2: Some(Direct(ZMM18)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 109, 69, 116, 230], OperandSize::Qword)
}

#[test]
fn vpcmpeqb_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQB, operand1: Some(Direct(K6)), operand2: Some(Direct(ZMM11)), operand3: Some(IndirectScaledDisplaced(RDI, Eight, 643890127, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 37, 75, 116, 52, 253, 207, 251, 96, 38], OperandSize::Qword)
}

