use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmadd231ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 184, 228], OperandSize::Dword)
}

#[test]
fn vfmadd231ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(EAX, ESI, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 184, 4, 112], OperandSize::Dword)
}

#[test]
fn vfmadd231ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 184, 231], OperandSize::Qword)
}

#[test]
fn vfmadd231ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RDI, Two, 1162894585, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 184, 180, 126, 249, 92, 80, 69], OperandSize::Qword)
}

#[test]
fn vfmadd231ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 184, 248], OperandSize::Dword)
}

#[test]
fn vfmadd231ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexed(EBX, ESI, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 184, 12, 179], OperandSize::Dword)
}

#[test]
fn vfmadd231ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 184, 244], OperandSize::Qword)
}

#[test]
fn vfmadd231ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RCX, Eight, 689478977, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 184, 132, 203, 65, 157, 24, 41], OperandSize::Qword)
}

#[test]
fn vfmadd231ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 141, 184, 231], OperandSize::Dword)
}

#[test]
fn vfmadd231ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(EDX, Two, 983449724, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 85, 141, 184, 44, 85, 124, 64, 158, 58], OperandSize::Dword)
}

#[test]
fn vfmadd231ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(EBX, EAX, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 93, 156, 184, 4, 67], OperandSize::Dword)
}

#[test]
fn vfmadd231ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PS, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM29)), operand3: Some(Direct(XMM21)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 162, 21, 131, 184, 205], OperandSize::Qword)
}

#[test]
fn vfmadd231ps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PS, operand1: Some(Direct(XMM25)), operand2: Some(Direct(XMM19)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RBX, Two, 397178414, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 98, 101, 134, 184, 140, 90, 46, 118, 172, 23], OperandSize::Qword)
}

#[test]
fn vfmadd231ps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PS, operand1: Some(Direct(XMM24)), operand2: Some(Direct(XMM31)), operand3: Some(IndirectScaledDisplaced(RSI, Four, 670139658, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 98, 5, 147, 184, 4, 181, 10, 133, 241, 39], OperandSize::Qword)
}

#[test]
fn vfmadd231ps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 85, 174, 184, 203], OperandSize::Dword)
}

#[test]
fn vfmadd231ps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectDisplaced(EDX, 640118828, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 169, 184, 186, 44, 112, 39, 38], OperandSize::Dword)
}

#[test]
fn vfmadd231ps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexed(EDX, EBX, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 109, 191, 184, 28, 154], OperandSize::Dword)
}

#[test]
fn vfmadd231ps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PS, operand1: Some(Direct(YMM30)), operand2: Some(Direct(YMM14)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 98, 13, 174, 184, 242], OperandSize::Qword)
}

#[test]
fn vfmadd231ps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM17)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RDX, Four, 1473328776, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 117, 163, 184, 180, 147, 136, 54, 209, 87], OperandSize::Qword)
}

#[test]
fn vfmadd231ps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PS, operand1: Some(Direct(YMM11)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexed(RSI, RBX, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 114, 125, 186, 184, 28, 222], OperandSize::Qword)
}

#[test]
fn vfmadd231ps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PS, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 109, 220, 184, 225], OperandSize::Dword)
}

#[test]
fn vfmadd231ps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PS, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EAX, Eight, 65249710, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 69, 204, 184, 164, 192, 174, 161, 227, 3], OperandSize::Dword)
}

#[test]
fn vfmadd231ps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PS, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, ECX, Two, 1237609537, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 69, 221, 184, 156, 72, 65, 108, 196, 73], OperandSize::Dword)
}

#[test]
fn vfmadd231ps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PS, operand1: Some(Direct(ZMM12)), operand2: Some(Direct(ZMM22)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 114, 77, 209, 184, 225], OperandSize::Qword)
}

#[test]
fn vfmadd231ps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PS, operand1: Some(Direct(ZMM23)), operand2: Some(Direct(ZMM19)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RAX, Eight, 762314430, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 226, 101, 196, 184, 188, 199, 190, 254, 111, 45], OperandSize::Qword)
}

#[test]
fn vfmadd231ps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADD231PS, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM22)), operand3: Some(IndirectScaledDisplaced(RSI, Two, 598228853, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 77, 210, 184, 44, 117, 117, 63, 168, 35], OperandSize::Qword)
}

