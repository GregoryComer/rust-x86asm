use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmsub213pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 217, 170, 211], OperandSize::Dword)
}

#[test]
fn vfmsub213pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexed(EAX, EBX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 241, 170, 52, 152], OperandSize::Dword)
}

#[test]
fn vfmsub213pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 233, 170, 215], OperandSize::Qword)
}

#[test]
fn vfmsub213pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(Indirect(RCX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 209, 170, 25], OperandSize::Qword)
}

#[test]
fn vfmsub213pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 197, 170, 207], OperandSize::Dword)
}

#[test]
fn vfmsub213pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexed(EAX, ESI, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 245, 170, 36, 240], OperandSize::Dword)
}

#[test]
fn vfmsub213pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 253, 170, 252], OperandSize::Qword)
}

#[test]
fn vfmsub213pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexed(RDX, RCX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 237, 170, 60, 202], OperandSize::Qword)
}

#[test]
fn vfmsub213pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 197, 137, 170, 239], OperandSize::Dword)
}

#[test]
fn vfmsub213pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EDX, Four, 1499295764, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 237, 138, 170, 140, 146, 20, 112, 93, 89], OperandSize::Dword)
}

#[test]
fn vfmsub213pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EDX, Eight, 1665639945, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 205, 159, 170, 188, 208, 9, 166, 71, 99], OperandSize::Dword)
}

#[test]
fn vfmsub213pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PD, operand1: Some(Direct(XMM28)), operand2: Some(Direct(XMM18)), operand3: Some(Direct(XMM15)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 66, 237, 135, 170, 231], OperandSize::Qword)
}

#[test]
fn vfmsub213pd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PD, operand1: Some(Direct(XMM19)), operand2: Some(Direct(XMM14)), operand3: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 226, 141, 143, 170, 24], OperandSize::Qword)
}

#[test]
fn vfmsub213pd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM11)), operand3: Some(IndirectDisplaced(RCX, 1248862468, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 165, 159, 170, 129, 4, 33, 112, 74], OperandSize::Qword)
}

#[test]
fn vfmsub213pd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 197, 169, 170, 234], OperandSize::Dword)
}

#[test]
fn vfmsub213pd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledDisplaced(EBX, Four, 936768821, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 197, 171, 170, 12, 157, 53, 245, 213, 55], OperandSize::Dword)
}

#[test]
fn vfmsub213pd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM4)), operand3: Some(Indirect(ECX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 221, 189, 170, 17], OperandSize::Dword)
}

#[test]
fn vfmsub213pd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PD, operand1: Some(Direct(YMM27)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM25)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 2, 205, 173, 170, 217], OperandSize::Qword)
}

#[test]
fn vfmsub213pd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM12)), operand3: Some(Indirect(RBX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 157, 169, 170, 51], OperandSize::Qword)
}

#[test]
fn vfmsub213pd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM16)), operand3: Some(IndirectScaledIndexed(RBX, RDX, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 253, 179, 170, 4, 83], OperandSize::Qword)
}

#[test]
fn vfmsub213pd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM7)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 221, 159, 170, 223], OperandSize::Dword)
}

#[test]
fn vfmsub213pd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM7)), operand3: Some(Indirect(EDX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 197, 205, 170, 58], OperandSize::Dword)
}

#[test]
fn vfmsub213pd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, ECX, Eight, 640292400, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 245, 217, 170, 132, 203, 48, 22, 42, 38], OperandSize::Dword)
}

#[test]
fn vfmsub213pd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PD, operand1: Some(Direct(ZMM27)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM19)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 34, 237, 186, 170, 219], OperandSize::Qword)
}

#[test]
fn vfmsub213pd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM27)), operand3: Some(IndirectDisplaced(RBX, 865706682, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 165, 196, 170, 131, 186, 162, 153, 51], OperandSize::Qword)
}

#[test]
fn vfmsub213pd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PD, operand1: Some(Direct(ZMM31)), operand2: Some(Direct(ZMM13)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RDX, Two, 1869360179, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 98, 149, 218, 170, 188, 80, 51, 44, 108, 111], OperandSize::Qword)
}

