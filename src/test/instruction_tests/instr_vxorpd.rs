use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vxorpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 87, 196], OperandSize::Dword)
}

#[test]
fn vxorpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(EDI, Four, 1206559918, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 87, 52, 189, 174, 164, 234, 71], OperandSize::Dword)
}

#[test]
fn vxorpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 87, 247], OperandSize::Qword)
}

#[test]
fn vxorpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(RDX, Two, 1889141485, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 87, 60, 85, 237, 2, 154, 112], OperandSize::Qword)
}

#[test]
fn vxorpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 87, 233], OperandSize::Dword)
}

#[test]
fn vxorpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectDisplaced(EDI, 1379486024, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 87, 191, 72, 73, 57, 82], OperandSize::Dword)
}

#[test]
fn vxorpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 87, 241], OperandSize::Qword)
}

#[test]
fn vxorpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RAX, Four, 383724092, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 87, 172, 131, 60, 42, 223, 22], OperandSize::Qword)
}

#[test]
fn vxorpd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 237, 142, 87, 246], OperandSize::Dword)
}

#[test]
fn vxorpd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(EAX, Eight, 60924740, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 229, 142, 87, 28, 197, 68, 163, 161, 3], OperandSize::Dword)
}

#[test]
fn vxorpd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectDisplaced(EAX, 513319316, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 221, 157, 87, 160, 148, 161, 152, 30], OperandSize::Dword)
}

#[test]
fn vxorpd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPD, operand1: Some(Direct(XMM26)), operand2: Some(Direct(XMM9)), operand3: Some(Direct(XMM27)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 1, 181, 143, 87, 211], OperandSize::Qword)
}

#[test]
fn vxorpd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPD, operand1: Some(Direct(XMM26)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(RCX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 97, 197, 143, 87, 17], OperandSize::Qword)
}

#[test]
fn vxorpd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM28)), operand3: Some(IndirectScaledIndexed(RDX, RCX, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 157, 149, 87, 28, 202], OperandSize::Qword)
}

#[test]
fn vxorpd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 221, 169, 87, 242], OperandSize::Dword)
}

#[test]
fn vxorpd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM3)), operand3: Some(Indirect(EDI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 229, 172, 87, 63], OperandSize::Dword)
}

#[test]
fn vxorpd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM2)), operand3: Some(Indirect(EDI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 237, 190, 87, 47], OperandSize::Dword)
}

#[test]
fn vxorpd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPD, operand1: Some(Direct(YMM25)), operand2: Some(Direct(YMM16)), operand3: Some(Direct(YMM14)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 65, 253, 161, 87, 206], OperandSize::Qword)
}

#[test]
fn vxorpd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPD, operand1: Some(Direct(YMM27)), operand2: Some(Direct(YMM6)), operand3: Some(Indirect(RCX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 97, 205, 173, 87, 25], OperandSize::Qword)
}

#[test]
fn vxorpd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM13)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RBX, Eight, 1112332926, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 149, 190, 87, 172, 216, 126, 218, 76, 66], OperandSize::Qword)
}

#[test]
fn vxorpd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 237, 205, 87, 204], OperandSize::Dword)
}

#[test]
fn vxorpd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledIndexed(EDI, EBX, Four, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 245, 201, 87, 20, 159], OperandSize::Dword)
}

#[test]
fn vxorpd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, ESI, Two, 1279828246, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 213, 222, 87, 188, 112, 22, 161, 72, 76], OperandSize::Dword)
}

#[test]
fn vxorpd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPD, operand1: Some(Direct(ZMM28)), operand2: Some(Direct(ZMM15)), operand3: Some(Direct(ZMM24)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 1, 133, 201, 87, 224], OperandSize::Qword)
}

#[test]
fn vxorpd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPD, operand1: Some(Direct(ZMM19)), operand2: Some(Direct(ZMM11)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RCX, Eight, 442805901, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 225, 165, 204, 87, 156, 207, 141, 174, 100, 26], OperandSize::Qword)
}

#[test]
fn vxorpd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPD, operand1: Some(Direct(ZMM9)), operand2: Some(Direct(ZMM11)), operand3: Some(IndirectDisplaced(RDX, 566100806, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 113, 165, 222, 87, 138, 70, 3, 190, 33], OperandSize::Qword)
}

