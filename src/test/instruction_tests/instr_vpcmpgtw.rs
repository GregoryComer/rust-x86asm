use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpcmpgtw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 101, 236], OperandSize::Dword)
}

#[test]
fn vpcmpgtw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexed(EBX, EAX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 101, 36, 131], OperandSize::Dword)
}

#[test]
fn vpcmpgtw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 101, 232], OperandSize::Qword)
}

#[test]
fn vpcmpgtw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectDisplaced(RBX, 1285060682, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 101, 163, 74, 120, 152, 76], OperandSize::Qword)
}

#[test]
fn vpcmpgtw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 101, 245], OperandSize::Dword)
}

#[test]
fn vpcmpgtw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectDisplaced(EBX, 614984053, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 101, 131, 117, 233, 167, 36], OperandSize::Dword)
}

#[test]
fn vpcmpgtw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 101, 200], OperandSize::Qword)
}

#[test]
fn vpcmpgtw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexed(RCX, RBX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 101, 12, 89], OperandSize::Qword)
}

#[test]
fn vpcmpgtw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTW, operand1: Some(Direct(K2)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 109, 11, 101, 213], OperandSize::Dword)
}

#[test]
fn vpcmpgtw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTW, operand1: Some(Direct(K1)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledDisplaced(ECX, Eight, 705328976, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 69, 14, 101, 12, 205, 80, 119, 10, 42], OperandSize::Dword)
}

#[test]
fn vpcmpgtw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTW, operand1: Some(Direct(K1)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM20)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 177, 77, 11, 101, 204], OperandSize::Qword)
}

#[test]
fn vpcmpgtw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTW, operand1: Some(Direct(K3)), operand2: Some(Direct(XMM19)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RCX, Four, 256862341, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 101, 3, 101, 156, 137, 133, 104, 79, 15], OperandSize::Qword)
}

#[test]
fn vpcmpgtw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTW, operand1: Some(Direct(K4)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 93, 42, 101, 225], OperandSize::Dword)
}

#[test]
fn vpcmpgtw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTW, operand1: Some(Direct(K1)), operand2: Some(Direct(YMM4)), operand3: Some(Indirect(EDI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 93, 46, 101, 15], OperandSize::Dword)
}

#[test]
fn vpcmpgtw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTW, operand1: Some(Direct(K1)), operand2: Some(Direct(YMM8)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 61, 42, 101, 200], OperandSize::Qword)
}

#[test]
fn vpcmpgtw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTW, operand1: Some(Direct(K2)), operand2: Some(Direct(YMM30)), operand3: Some(IndirectDisplaced(RAX, 275270546, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 13, 37, 101, 144, 146, 75, 104, 16], OperandSize::Qword)
}

#[test]
fn vpcmpgtw_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTW, operand1: Some(Direct(K3)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 117, 77, 101, 220], OperandSize::Dword)
}

#[test]
fn vpcmpgtw_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTW, operand1: Some(Direct(K5)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectDisplaced(EDX, 334317554, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 109, 74, 101, 170, 242, 71, 237, 19], OperandSize::Dword)
}

#[test]
fn vpcmpgtw_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTW, operand1: Some(Direct(K7)), operand2: Some(Direct(ZMM14)), operand3: Some(Direct(ZMM11)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 209, 13, 78, 101, 251], OperandSize::Qword)
}

#[test]
fn vpcmpgtw_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTW, operand1: Some(Direct(K2)), operand2: Some(Direct(ZMM13)), operand3: Some(IndirectScaledIndexed(RCX, RDX, Two, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 21, 74, 101, 20, 81], OperandSize::Qword)
}

