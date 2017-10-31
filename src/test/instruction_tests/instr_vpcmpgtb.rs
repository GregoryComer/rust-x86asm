use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpcmpgtb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTB, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 100, 225], OperandSize::Dword)
}

#[test]
fn vpcmpgtb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EBX, Four, 99650900, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 100, 180, 152, 84, 141, 240, 5], OperandSize::Dword)
}

#[test]
fn vpcmpgtb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTB, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 100, 250], OperandSize::Qword)
}

#[test]
fn vpcmpgtb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTB, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(RBX, 1204583324, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 100, 171, 156, 123, 204, 71], OperandSize::Qword)
}

#[test]
fn vpcmpgtb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTB, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 100, 223], OperandSize::Dword)
}

#[test]
fn vpcmpgtb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTB, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledDisplaced(EDI, Four, 1121776670, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 100, 4, 189, 30, 244, 220, 66], OperandSize::Dword)
}

#[test]
fn vpcmpgtb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTB, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 100, 211], OperandSize::Qword)
}

#[test]
fn vpcmpgtb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTB, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM2)), operand3: Some(Indirect(RDX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 100, 26], OperandSize::Qword)
}

#[test]
fn vpcmpgtb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTB, operand1: Some(Direct(K7)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 101, 13, 100, 251], OperandSize::Dword)
}

#[test]
fn vpcmpgtb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTB, operand1: Some(Direct(K6)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectDisplaced(EDI, 847890200, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 93, 13, 100, 183, 24, 199, 137, 50], OperandSize::Dword)
}

#[test]
fn vpcmpgtb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTB, operand1: Some(Direct(K4)), operand2: Some(Direct(XMM31)), operand3: Some(Direct(XMM18)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 177, 5, 3, 100, 226], OperandSize::Qword)
}

#[test]
fn vpcmpgtb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTB, operand1: Some(Direct(K3)), operand2: Some(Direct(XMM27)), operand3: Some(IndirectScaledDisplaced(RCX, Four, 632924725, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 37, 4, 100, 28, 141, 53, 170, 185, 37], OperandSize::Qword)
}

#[test]
fn vpcmpgtb_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTB, operand1: Some(Direct(K1)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 85, 42, 100, 202], OperandSize::Dword)
}

#[test]
fn vpcmpgtb_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTB, operand1: Some(Direct(K4)), operand2: Some(Direct(YMM3)), operand3: Some(Indirect(EDI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 101, 44, 100, 39], OperandSize::Dword)
}

#[test]
fn vpcmpgtb_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTB, operand1: Some(Direct(K3)), operand2: Some(Direct(YMM23)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 69, 33, 100, 216], OperandSize::Qword)
}

#[test]
fn vpcmpgtb_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTB, operand1: Some(Direct(K2)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RDX, Two, 1857443336, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 117, 46, 100, 148, 81, 8, 86, 182, 110], OperandSize::Qword)
}

#[test]
fn vpcmpgtb_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTB, operand1: Some(Direct(K4)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 85, 76, 100, 230], OperandSize::Dword)
}

#[test]
fn vpcmpgtb_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTB, operand1: Some(Direct(K6)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledIndexed(ECX, EAX, Two, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 117, 73, 100, 52, 65], OperandSize::Dword)
}

#[test]
fn vpcmpgtb_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTB, operand1: Some(Direct(K4)), operand2: Some(Direct(ZMM24)), operand3: Some(Direct(ZMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 61, 69, 100, 229], OperandSize::Qword)
}

#[test]
fn vpcmpgtb_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTB, operand1: Some(Direct(K4)), operand2: Some(Direct(ZMM15)), operand3: Some(Indirect(RBX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 5, 79, 100, 35], OperandSize::Qword)
}

