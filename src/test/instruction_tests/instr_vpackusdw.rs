use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpackusdw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSDW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 43, 248], OperandSize::Dword)
}

#[test]
fn vpackusdw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSDW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledDisplaced(EBX, Four, 1720554741, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 43, 52, 157, 245, 148, 141, 102], OperandSize::Dword)
}

#[test]
fn vpackusdw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSDW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 43, 216], OperandSize::Qword)
}

#[test]
fn vpackusdw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSDW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(RAX, 210640854, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 43, 184, 214, 31, 142, 12], OperandSize::Qword)
}

#[test]
fn vpackusdw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSDW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 43, 208], OperandSize::Dword)
}

#[test]
fn vpackusdw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSDW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM7)), operand3: Some(Indirect(EDX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 43, 2], OperandSize::Dword)
}

#[test]
fn vpackusdw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSDW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 43, 193], OperandSize::Qword)
}

#[test]
fn vpackusdw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSDW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectDisplaced(RAX, 1242974735, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 43, 128, 15, 74, 22, 74], OperandSize::Qword)
}

#[test]
fn vpackusdw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSDW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 109, 140, 43, 204], OperandSize::Dword)
}

#[test]
fn vpackusdw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSDW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexed(ESI, EBX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 117, 143, 43, 12, 222], OperandSize::Dword)
}

#[test]
fn vpackusdw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSDW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EDX, Four, 1552971929, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 109, 155, 43, 156, 147, 153, 120, 144, 92], OperandSize::Dword)
}

#[test]
fn vpackusdw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSDW, operand1: Some(Direct(XMM30)), operand2: Some(Direct(XMM17)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 98, 117, 133, 43, 240], OperandSize::Qword)
}

#[test]
fn vpackusdw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSDW, operand1: Some(Direct(XMM31)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledDisplaced(RDX, Eight, 1165983154, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 98, 117, 140, 43, 60, 213, 178, 125, 127, 69], OperandSize::Qword)
}

#[test]
fn vpackusdw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSDW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM19)), operand3: Some(IndirectScaledDisplaced(RDI, Eight, 1486744094, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 101, 148, 43, 12, 253, 30, 234, 157, 88], OperandSize::Qword)
}

#[test]
fn vpackusdw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSDW, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 117, 203, 43, 230], OperandSize::Dword)
}

#[test]
fn vpackusdw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSDW, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, ESI, Four, 1428921081, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 101, 204, 43, 172, 182, 249, 154, 43, 85], OperandSize::Dword)
}

#[test]
fn vpackusdw_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSDW, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM4)), operand3: Some(Indirect(EBX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 93, 219, 43, 59], OperandSize::Dword)
}

#[test]
fn vpackusdw_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSDW, operand1: Some(Direct(ZMM25)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM15)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 66, 85, 206, 43, 207], OperandSize::Qword)
}

#[test]
fn vpackusdw_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSDW, operand1: Some(Direct(ZMM24)), operand2: Some(Direct(ZMM11)), operand3: Some(IndirectDisplaced(RBX, 1795707662, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 98, 37, 207, 43, 131, 14, 83, 8, 107], OperandSize::Qword)
}

#[test]
fn vpackusdw_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSDW, operand1: Some(Direct(ZMM20)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledIndexed(RCX, RDI, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 226, 117, 223, 43, 36, 185], OperandSize::Qword)
}

