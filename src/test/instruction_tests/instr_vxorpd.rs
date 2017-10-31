use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vxorpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 87, 202], OperandSize::Dword)
}

#[test]
fn vxorpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexed(EAX, EDI, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 87, 60, 248], OperandSize::Dword)
}

#[test]
fn vxorpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 87, 214], OperandSize::Qword)
}

#[test]
fn vxorpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexed(RDX, RCX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 87, 60, 138], OperandSize::Qword)
}

#[test]
fn vxorpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 87, 193], OperandSize::Dword)
}

#[test]
fn vxorpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexed(EAX, ESI, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 87, 36, 240], OperandSize::Dword)
}

#[test]
fn vxorpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 87, 209], OperandSize::Qword)
}

#[test]
fn vxorpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledDisplaced(RBX, Eight, 1811556573, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 87, 12, 221, 221, 40, 250, 107], OperandSize::Qword)
}

#[test]
fn vxorpd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 221, 138, 87, 251], OperandSize::Dword)
}

#[test]
fn vxorpd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexed(ECX, EAX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 229, 137, 87, 28, 65], OperandSize::Dword)
}

#[test]
fn vxorpd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EDX, Four, 2119553866, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 237, 159, 87, 188, 146, 74, 211, 85, 126], OperandSize::Dword)
}

#[test]
fn vxorpd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPD, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM19)), operand3: Some(Direct(XMM29)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 17, 229, 132, 87, 213], OperandSize::Qword)
}

#[test]
fn vxorpd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPD, operand1: Some(Direct(XMM9)), operand2: Some(Direct(XMM15)), operand3: Some(IndirectScaledIndexed(RDX, RDX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 113, 133, 143, 87, 12, 82], OperandSize::Qword)
}

#[test]
fn vxorpd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPD, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM31)), operand3: Some(IndirectDisplaced(RDX, 1171620598, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 113, 133, 146, 87, 146, 246, 130, 213, 69], OperandSize::Qword)
}

#[test]
fn vxorpd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 197, 172, 87, 207], OperandSize::Dword)
}

#[test]
fn vxorpd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, ECX, Eight, 974354233, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 197, 172, 87, 172, 200, 57, 119, 19, 58], OperandSize::Dword)
}

#[test]
fn vxorpd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM5)), operand3: Some(Indirect(EAX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 213, 188, 87, 40], OperandSize::Dword)
}

#[test]
fn vxorpd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPD, operand1: Some(Direct(YMM21)), operand2: Some(Direct(YMM11)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 225, 165, 174, 87, 232], OperandSize::Qword)
}

#[test]
fn vxorpd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPD, operand1: Some(Direct(YMM13)), operand2: Some(Direct(YMM29)), operand3: Some(IndirectDisplaced(RBX, 1564542029, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 113, 149, 161, 87, 171, 77, 4, 65, 93], OperandSize::Qword)
}

#[test]
fn vxorpd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledDisplaced(RAX, Two, 1015474028, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 221, 187, 87, 20, 69, 108, 231, 134, 60], OperandSize::Qword)
}

#[test]
fn vxorpd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 197, 206, 87, 216], OperandSize::Dword)
}

#[test]
fn vxorpd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EAX, Eight, 1904054678, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 197, 205, 87, 188, 198, 150, 145, 125, 113], OperandSize::Dword)
}

#[test]
fn vxorpd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectDisplaced(EDX, 395000604, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 213, 221, 87, 146, 28, 59, 139, 23], OperandSize::Dword)
}

#[test]
fn vxorpd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPD, operand1: Some(Direct(ZMM30)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 97, 213, 203, 87, 244], OperandSize::Qword)
}

#[test]
fn vxorpd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPD, operand1: Some(Direct(ZMM17)), operand2: Some(Direct(ZMM26)), operand3: Some(IndirectScaledDisplaced(RSI, Eight, 1628539551, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 225, 173, 194, 87, 12, 245, 159, 138, 17, 97], OperandSize::Qword)
}

#[test]
fn vxorpd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPD, operand1: Some(Direct(ZMM10)), operand2: Some(Direct(ZMM26)), operand3: Some(IndirectScaledDisplaced(RSI, Two, 128997004, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 113, 173, 212, 87, 20, 117, 140, 86, 176, 7], OperandSize::Qword)
}

