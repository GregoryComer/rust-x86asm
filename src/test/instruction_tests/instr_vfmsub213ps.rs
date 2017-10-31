use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmsub213ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 170, 236], OperandSize::Dword)
}

#[test]
fn vfmsub213ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EDX, Eight, 482851875, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 170, 132, 209, 35, 188, 199, 28], OperandSize::Dword)
}

#[test]
fn vfmsub213ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 170, 204], OperandSize::Qword)
}

#[test]
fn vfmsub213ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(Indirect(RDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 170, 58], OperandSize::Qword)
}

#[test]
fn vfmsub213ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 170, 222], OperandSize::Dword)
}

#[test]
fn vfmsub213ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledDisplaced(EAX, Two, 1290048387, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 93, 170, 44, 69, 131, 147, 228, 76], OperandSize::Dword)
}

#[test]
fn vfmsub213ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 77, 170, 235], OperandSize::Qword)
}

#[test]
fn vfmsub213ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectDisplaced(RCX, 1998410438, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 170, 161, 198, 82, 29, 119], OperandSize::Qword)
}

#[test]
fn vfmsub213ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 109, 142, 170, 227], OperandSize::Dword)
}

#[test]
fn vfmsub213ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledDisplaced(EDI, Two, 1679810412, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 117, 139, 170, 44, 125, 108, 223, 31, 100], OperandSize::Dword)
}

#[test]
fn vfmsub213ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectDisplaced(EDI, 92016613, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 125, 159, 170, 175, 229, 15, 124, 5], OperandSize::Dword)
}

#[test]
fn vfmsub213ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PS, operand1: Some(Direct(XMM31)), operand2: Some(Direct(XMM28)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 98, 29, 134, 170, 249], OperandSize::Qword)
}

#[test]
fn vfmsub213ps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PS, operand1: Some(Direct(XMM13)), operand2: Some(Direct(XMM16)), operand3: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 114, 125, 131, 170, 40], OperandSize::Qword)
}

#[test]
fn vfmsub213ps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PS, operand1: Some(Direct(XMM31)), operand2: Some(Direct(XMM10)), operand3: Some(Indirect(RCX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 98, 45, 157, 170, 57], OperandSize::Qword)
}

#[test]
fn vfmsub213ps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 77, 173, 170, 201], OperandSize::Dword)
}

#[test]
fn vfmsub213ps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledDisplaced(EDI, Two, 1322759666, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 85, 173, 170, 20, 125, 242, 181, 215, 78], OperandSize::Dword)
}

#[test]
fn vfmsub213ps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectDisplaced(EDX, 2076095980, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 69, 187, 170, 138, 236, 181, 190, 123], OperandSize::Dword)
}

#[test]
fn vfmsub213ps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PS, operand1: Some(Direct(YMM28)), operand2: Some(Direct(YMM11)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 98, 37, 172, 170, 226], OperandSize::Qword)
}

#[test]
fn vfmsub213ps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PS, operand1: Some(Direct(YMM17)), operand2: Some(Direct(YMM15)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RBX, Two, 847709372, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 226, 5, 174, 170, 140, 90, 188, 4, 135, 50], OperandSize::Qword)
}

#[test]
fn vfmsub213ps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PS, operand1: Some(Direct(YMM9)), operand2: Some(Direct(YMM26)), operand3: Some(IndirectDisplaced(RCX, 236528231, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 114, 45, 179, 170, 137, 103, 34, 25, 14], OperandSize::Qword)
}

#[test]
fn vfmsub213ps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PS, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM2)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 101, 255, 170, 234], OperandSize::Dword)
}

#[test]
fn vfmsub213ps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PS, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EAX, Two, 1908409183, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 85, 204, 170, 164, 65, 95, 3, 192, 113], OperandSize::Dword)
}

#[test]
fn vfmsub213ps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PS, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectDisplaced(EAX, 1541857169, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 109, 220, 170, 128, 145, 223, 230, 91], OperandSize::Dword)
}

#[test]
fn vfmsub213ps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PS, operand1: Some(Direct(ZMM10)), operand2: Some(Direct(ZMM17)), operand3: Some(Direct(ZMM29)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 18, 117, 247, 170, 213], OperandSize::Qword)
}

#[test]
fn vfmsub213ps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PS, operand1: Some(Direct(ZMM20)), operand2: Some(Direct(ZMM31)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RDX, Two, 358335554, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 226, 5, 198, 170, 164, 80, 66, 196, 91, 21], OperandSize::Qword)
}

#[test]
fn vfmsub213ps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PS, operand1: Some(Direct(ZMM31)), operand2: Some(Direct(ZMM3)), operand3: Some(Indirect(RCX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 98, 101, 217, 170, 57], OperandSize::Qword)
}

