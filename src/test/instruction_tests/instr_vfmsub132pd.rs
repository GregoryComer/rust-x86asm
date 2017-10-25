use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmsub132pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 241, 154, 196], OperandSize::Dword)
}

#[test]
fn vfmsub132pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectDisplaced(EAX, 1504502089, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 249, 154, 152, 73, 225, 172, 89], OperandSize::Dword)
}

#[test]
fn vfmsub132pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 241, 154, 198], OperandSize::Qword)
}

#[test]
fn vfmsub132pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(RCX, 1886608706, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 241, 154, 185, 66, 93, 115, 112], OperandSize::Qword)
}

#[test]
fn vfmsub132pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 245, 154, 210], OperandSize::Dword)
}

#[test]
fn vfmsub132pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledDisplaced(EDX, Four, 275656782, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 221, 154, 20, 149, 78, 48, 110, 16], OperandSize::Dword)
}

#[test]
fn vfmsub132pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 237, 154, 230], OperandSize::Qword)
}

#[test]
fn vfmsub132pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RAX, Eight, 589814213, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 245, 154, 140, 192, 197, 217, 39, 35], OperandSize::Qword)
}

#[test]
fn vfmsub132pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 229, 138, 154, 215], OperandSize::Dword)
}

#[test]
fn vfmsub132pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledDisplaced(ECX, Eight, 285502440, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 205, 143, 154, 20, 205, 232, 107, 4, 17], OperandSize::Dword)
}

#[test]
fn vfmsub132pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectDisplaced(EDI, 935036340, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 205, 157, 154, 159, 180, 133, 187, 55], OperandSize::Dword)
}

#[test]
fn vfmsub132pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PD, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM11)), operand3: Some(Direct(XMM24)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 130, 165, 137, 154, 200], OperandSize::Qword)
}

#[test]
fn vfmsub132pd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PD, operand1: Some(Direct(XMM24)), operand2: Some(Direct(XMM15)), operand3: Some(Indirect(RBX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 98, 133, 139, 154, 3], OperandSize::Qword)
}

#[test]
fn vfmsub132pd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PD, operand1: Some(Direct(XMM8)), operand2: Some(Direct(XMM11)), operand3: Some(IndirectScaledDisplaced(RDI, Two, 442912269, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 114, 165, 157, 154, 4, 125, 13, 78, 102, 26], OperandSize::Qword)
}

#[test]
fn vfmsub132pd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 229, 173, 154, 211], OperandSize::Dword)
}

#[test]
fn vfmsub132pd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexed(ESI, EBX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 237, 169, 154, 12, 222], OperandSize::Dword)
}

#[test]
fn vfmsub132pd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexed(EBX, EDI, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 229, 185, 154, 28, 123], OperandSize::Dword)
}

#[test]
fn vfmsub132pd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PD, operand1: Some(Direct(YMM11)), operand2: Some(Direct(YMM9)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 114, 181, 172, 154, 220], OperandSize::Qword)
}

#[test]
fn vfmsub132pd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM13)), operand3: Some(IndirectDisplaced(RCX, 731919960, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 149, 171, 154, 145, 88, 54, 160, 43], OperandSize::Qword)
}

#[test]
fn vfmsub132pd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM18)), operand3: Some(IndirectDisplaced(RDI, 763290496, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 237, 181, 154, 191, 128, 227, 126, 45], OperandSize::Qword)
}

#[test]
fn vfmsub132pd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM2)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 229, 250, 154, 226], OperandSize::Dword)
}

#[test]
fn vfmsub132pd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EBX, Two, 233603410, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 229, 202, 154, 148, 91, 82, 129, 236, 13], OperandSize::Dword)
}

#[test]
fn vfmsub132pd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectDisplaced(EBX, 209130053, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 253, 218, 154, 187, 69, 18, 119, 12], OperandSize::Dword)
}

#[test]
fn vfmsub132pd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PD, operand1: Some(Direct(ZMM26)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM31)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 2, 229, 223, 154, 215], OperandSize::Qword)
}

#[test]
fn vfmsub132pd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM27)), operand3: Some(IndirectDisplaced(RAX, 192114673, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 165, 198, 154, 184, 241, 111, 115, 11], OperandSize::Qword)
}

#[test]
fn vfmsub132pd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PD, operand1: Some(Direct(ZMM15)), operand2: Some(Direct(ZMM26)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RDX, Two, 1154795467, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 114, 173, 213, 154, 188, 82, 203, 199, 212, 68], OperandSize::Qword)
}

