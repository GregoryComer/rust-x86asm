use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vxorpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 87, 245], OperandSize::Dword)
}

#[test]
fn vxorpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: Some(Indirect(EAX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 87, 16], OperandSize::Dword)
}

#[test]
fn vxorpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 87, 227], OperandSize::Qword)
}

#[test]
fn vxorpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(RCX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 87, 49], OperandSize::Qword)
}

#[test]
fn vxorpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 87, 222], OperandSize::Dword)
}

#[test]
fn vxorpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledDisplaced(EAX, Eight, 111459017, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 87, 44, 197, 201, 186, 164, 6], OperandSize::Dword)
}

#[test]
fn vxorpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 87, 194], OperandSize::Qword)
}

#[test]
fn vxorpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectDisplaced(RCX, 981650666, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 87, 153, 234, 204, 130, 58], OperandSize::Qword)
}

#[test]
fn vxorpd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 221, 139, 87, 209], OperandSize::Dword)
}

#[test]
fn vxorpd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EAX, Two, 2016349233, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 245, 142, 87, 172, 65, 49, 12, 47, 120], OperandSize::Dword)
}

#[test]
fn vxorpd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(EDI, EBX, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 221, 158, 87, 20, 95], OperandSize::Dword)
}

#[test]
fn vxorpd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPD, operand1: Some(Direct(XMM27)), operand2: Some(Direct(XMM30)), operand3: Some(Direct(XMM19)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 33, 141, 129, 87, 219], OperandSize::Qword)
}

#[test]
fn vxorpd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPD, operand1: Some(Direct(XMM25)), operand2: Some(Direct(XMM12)), operand3: Some(Indirect(RCX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 97, 157, 137, 87, 9], OperandSize::Qword)
}

#[test]
fn vxorpd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPD, operand1: Some(Direct(XMM31)), operand2: Some(Direct(XMM24)), operand3: Some(Indirect(RCX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 97, 189, 149, 87, 57], OperandSize::Qword)
}

#[test]
fn vxorpd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 229, 169, 87, 203], OperandSize::Dword)
}

#[test]
fn vxorpd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EDX, Two, 555874915, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 213, 174, 87, 172, 81, 99, 250, 33, 33], OperandSize::Dword)
}

#[test]
fn vxorpd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledDisplaced(ECX, Four, 1403746000, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 221, 186, 87, 20, 141, 208, 118, 171, 83], OperandSize::Dword)
}

#[test]
fn vxorpd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPD, operand1: Some(Direct(YMM28)), operand2: Some(Direct(YMM19)), operand3: Some(Direct(YMM22)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 33, 229, 165, 87, 230], OperandSize::Qword)
}

#[test]
fn vxorpd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPD, operand1: Some(Direct(YMM9)), operand2: Some(Direct(YMM11)), operand3: Some(IndirectScaledDisplaced(RAX, Four, 1470318065, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 113, 165, 172, 87, 12, 133, 241, 69, 163, 87], OperandSize::Qword)
}

#[test]
fn vxorpd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPD, operand1: Some(Direct(YMM24)), operand2: Some(Direct(YMM18)), operand3: Some(IndirectScaledIndexed(RSI, RCX, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 97, 237, 181, 87, 4, 206], OperandSize::Qword)
}

#[test]
fn vxorpd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 213, 205, 87, 201], OperandSize::Dword)
}

#[test]
fn vxorpd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledIndexed(EAX, EDX, Eight, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 245, 203, 87, 52, 208], OperandSize::Dword)
}

#[test]
fn vxorpd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, ECX, Eight, 534130756, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 253, 222, 87, 164, 201, 68, 48, 214, 31], OperandSize::Dword)
}

#[test]
fn vxorpd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPD, operand1: Some(Direct(ZMM17)), operand2: Some(Direct(ZMM28)), operand3: Some(Direct(ZMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 225, 157, 197, 87, 205], OperandSize::Qword)
}

#[test]
fn vxorpd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPD, operand1: Some(Direct(ZMM12)), operand2: Some(Direct(ZMM6)), operand3: Some(Indirect(RCX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 113, 205, 204, 87, 33], OperandSize::Qword)
}

#[test]
fn vxorpd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VXORPD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM11)), operand3: Some(Indirect(RSI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 165, 222, 87, 38], OperandSize::Qword)
}

