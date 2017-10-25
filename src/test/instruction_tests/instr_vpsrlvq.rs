use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpsrlvq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 249, 69, 222], OperandSize::Dword)
}

#[test]
fn vpsrlvq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(Indirect(EDI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 217, 69, 47], OperandSize::Dword)
}

#[test]
fn vpsrlvq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 249, 69, 215], OperandSize::Qword)
}

#[test]
fn vpsrlvq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(RDI, Eight, 1818397689, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 225, 69, 44, 253, 249, 139, 98, 108], OperandSize::Qword)
}

#[test]
fn vpsrlvq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVQ, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 213, 69, 196], OperandSize::Dword)
}

#[test]
fn vpsrlvq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledDisplaced(EDX, Four, 1670232395, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 213, 69, 36, 149, 75, 185, 141, 99], OperandSize::Dword)
}

#[test]
fn vpsrlvq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVQ, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 205, 69, 239], OperandSize::Qword)
}

#[test]
fn vpsrlvq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVQ, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledDisplaced(RSI, Eight, 482192583, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 213, 69, 28, 245, 199, 172, 189, 28], OperandSize::Qword)
}

#[test]
fn vpsrlvq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 229, 138, 69, 232], OperandSize::Dword)
}

#[test]
fn vpsrlvq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, ESI, Two, 1651394427, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 245, 137, 69, 140, 113, 123, 71, 110, 98], OperandSize::Dword)
}

#[test]
fn vpsrlvq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EBX, Four, 995812707, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 253, 155, 69, 148, 159, 99, 229, 90, 59], OperandSize::Dword)
}

#[test]
fn vpsrlvq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVQ, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM19)), operand3: Some(Direct(XMM17)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 50, 229, 132, 69, 209], OperandSize::Qword)
}

#[test]
fn vpsrlvq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVQ, operand1: Some(Direct(XMM27)), operand2: Some(Direct(XMM14)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RDI, Two, 161725200, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 98, 141, 140, 69, 156, 121, 16, 187, 163, 9], OperandSize::Qword)
}

#[test]
fn vpsrlvq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVQ, operand1: Some(Direct(XMM14)), operand2: Some(Direct(XMM31)), operand3: Some(IndirectScaledDisplaced(RBX, Eight, 1312689835, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 114, 133, 149, 69, 52, 221, 171, 14, 62, 78], OperandSize::Qword)
}

#[test]
fn vpsrlvq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVQ, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 229, 170, 69, 249], OperandSize::Dword)
}

#[test]
fn vpsrlvq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledDisplaced(EBX, Two, 1130088265, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 205, 173, 69, 52, 93, 73, 199, 91, 67], OperandSize::Dword)
}

#[test]
fn vpsrlvq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EBX, Four, 1864763166, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 205, 188, 69, 164, 158, 30, 7, 38, 111], OperandSize::Dword)
}

#[test]
fn vpsrlvq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVQ, operand1: Some(Direct(YMM12)), operand2: Some(Direct(YMM12)), operand3: Some(Direct(YMM14)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 82, 157, 169, 69, 230], OperandSize::Qword)
}

#[test]
fn vpsrlvq_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM19)), operand3: Some(IndirectScaledIndexed(RSI, RDI, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 229, 164, 69, 36, 126], OperandSize::Qword)
}

#[test]
fn vpsrlvq_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM7)), operand3: Some(Indirect(RDI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 197, 189, 69, 55], OperandSize::Qword)
}

#[test]
fn vpsrlvq_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVQ, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 245, 205, 69, 226], OperandSize::Dword)
}

#[test]
fn vpsrlvq_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVQ, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectDisplaced(EDI, 1121347806, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 197, 207, 69, 175, 222, 104, 214, 66], OperandSize::Dword)
}

#[test]
fn vpsrlvq_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVQ, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM5)), operand3: Some(Indirect(EAX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 213, 219, 69, 8], OperandSize::Dword)
}

#[test]
fn vpsrlvq_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVQ, operand1: Some(Direct(ZMM24)), operand2: Some(Direct(ZMM28)), operand3: Some(Direct(ZMM8)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 66, 157, 196, 69, 192], OperandSize::Qword)
}

#[test]
fn vpsrlvq_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVQ, operand1: Some(Direct(ZMM15)), operand2: Some(Direct(ZMM13)), operand3: Some(IndirectScaledIndexed(RDI, RAX, Four, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 114, 149, 201, 69, 60, 135], OperandSize::Qword)
}

#[test]
fn vpsrlvq_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVQ, operand1: Some(Direct(ZMM12)), operand2: Some(Direct(ZMM29)), operand3: Some(Indirect(RCX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 114, 149, 209, 69, 33], OperandSize::Qword)
}

