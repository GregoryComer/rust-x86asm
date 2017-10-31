use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpcmpeqb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQB, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 116, 227], OperandSize::Dword)
}

#[test]
fn vpcmpeqb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQB, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexed(EDX, EAX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 116, 28, 66], OperandSize::Dword)
}

#[test]
fn vpcmpeqb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 116, 192], OperandSize::Qword)
}

#[test]
fn vpcmpeqb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQB, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectDisplaced(RDX, 1174176774, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 116, 186, 6, 132, 252, 69], OperandSize::Qword)
}

#[test]
fn vpcmpeqb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQB, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 116, 224], OperandSize::Dword)
}

#[test]
fn vpcmpeqb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQB, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectDisplaced(EBX, 1104793996, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 116, 139, 140, 209, 217, 65], OperandSize::Dword)
}

#[test]
fn vpcmpeqb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQB, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 116, 194], OperandSize::Qword)
}

#[test]
fn vpcmpeqb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQB, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectDisplaced(RBX, 1255049295, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 116, 163, 79, 136, 206, 74], OperandSize::Qword)
}

#[test]
fn vpcmpeqb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQB, operand1: Some(Direct(K2)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 77, 9, 116, 212], OperandSize::Dword)
}

#[test]
fn vpcmpeqb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQB, operand1: Some(Direct(K6)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(EAX, EDX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 85, 11, 116, 52, 144], OperandSize::Dword)
}

#[test]
fn vpcmpeqb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQB, operand1: Some(Direct(K3)), operand2: Some(Direct(XMM27)), operand3: Some(Direct(XMM28)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 145, 37, 5, 116, 220], OperandSize::Qword)
}

#[test]
fn vpcmpeqb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQB, operand1: Some(Direct(K4)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(RDI, 1550210114, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 117, 9, 116, 167, 66, 84, 102, 92], OperandSize::Qword)
}

#[test]
fn vpcmpeqb_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQB, operand1: Some(Direct(K2)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 69, 47, 116, 210], OperandSize::Dword)
}

#[test]
fn vpcmpeqb_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQB, operand1: Some(Direct(K7)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledDisplaced(EDI, Eight, 179349179, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 109, 43, 116, 60, 253, 187, 166, 176, 10], OperandSize::Dword)
}

#[test]
fn vpcmpeqb_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQB, operand1: Some(Direct(K2)), operand2: Some(Direct(YMM21)), operand3: Some(Direct(YMM9)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 209, 85, 37, 116, 209], OperandSize::Qword)
}

#[test]
fn vpcmpeqb_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQB, operand1: Some(Direct(K5)), operand2: Some(Direct(YMM21)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RDX, Four, 1350755851, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 85, 36, 116, 172, 144, 11, 230, 130, 80], OperandSize::Qword)
}

#[test]
fn vpcmpeqb_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQB, operand1: Some(Direct(K5)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 77, 76, 116, 238], OperandSize::Dword)
}

#[test]
fn vpcmpeqb_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQB, operand1: Some(Direct(K7)), operand2: Some(Direct(ZMM1)), operand3: Some(Indirect(EBX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 117, 78, 116, 59], OperandSize::Dword)
}

#[test]
fn vpcmpeqb_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQB, operand1: Some(Direct(K3)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM11)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 209, 69, 74, 116, 219], OperandSize::Qword)
}

#[test]
fn vpcmpeqb_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQB, operand1: Some(Direct(K1)), operand2: Some(Direct(ZMM9)), operand3: Some(IndirectDisplaced(RDI, 831342262, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 53, 74, 116, 143, 182, 70, 141, 49], OperandSize::Qword)
}

