use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmsub132ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 154, 239], OperandSize::Dword)
}

#[test]
fn vfmsub132ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(EBX, EAX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 154, 44, 195], OperandSize::Dword)
}

#[test]
fn vfmsub132ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 154, 213], OperandSize::Qword)
}

#[test]
fn vfmsub132ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RAX, Two, 1678585432, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 154, 188, 67, 88, 46, 13, 100], OperandSize::Qword)
}

#[test]
fn vfmsub132ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 154, 213], OperandSize::Dword)
}

#[test]
fn vfmsub132ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexed(EAX, EBX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 154, 4, 88], OperandSize::Dword)
}

#[test]
fn vfmsub132ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 154, 227], OperandSize::Qword)
}

#[test]
fn vfmsub132ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RAX, Two, 911718388, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 93, 154, 188, 65, 244, 183, 87, 54], OperandSize::Qword)
}

#[test]
fn vfmsub132ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 77, 143, 154, 209], OperandSize::Dword)
}

#[test]
fn vfmsub132ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EAX, Eight, 1558531562, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 85, 143, 154, 172, 192, 234, 77, 229, 92], OperandSize::Dword)
}

#[test]
fn vfmsub132ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectDisplaced(EDX, 847336755, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 69, 156, 154, 138, 51, 85, 129, 50], OperandSize::Dword)
}

#[test]
fn vfmsub132ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 93, 143, 154, 231], OperandSize::Qword)
}

#[test]
fn vfmsub132ps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PS, operand1: Some(Direct(XMM8)), operand2: Some(Direct(XMM28)), operand3: Some(IndirectScaledDisplaced(RDI, Eight, 1849058895, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 114, 29, 129, 154, 4, 253, 79, 102, 54, 110], OperandSize::Qword)
}

#[test]
fn vfmsub132ps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PS, operand1: Some(Direct(XMM26)), operand2: Some(Direct(XMM15)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RCX, Eight, 994270726, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 98, 5, 159, 154, 148, 200, 6, 94, 67, 59], OperandSize::Qword)
}

#[test]
fn vfmsub132ps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 101, 173, 154, 212], OperandSize::Dword)
}

#[test]
fn vfmsub132ps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EAX, Eight, 1131433536, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 93, 173, 154, 156, 193, 64, 78, 112, 67], OperandSize::Dword)
}

#[test]
fn vfmsub132ps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexed(EDI, ECX, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 77, 188, 154, 36, 207], OperandSize::Dword)
}

#[test]
fn vfmsub132ps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM22)), operand3: Some(Direct(YMM19)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 178, 77, 164, 154, 203], OperandSize::Qword)
}

#[test]
fn vfmsub132ps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM6)), operand3: Some(Indirect(RBX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 77, 175, 154, 19], OperandSize::Qword)
}

#[test]
fn vfmsub132ps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PS, operand1: Some(Direct(YMM13)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexed(RCX, RAX, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 114, 117, 187, 154, 44, 65], OperandSize::Qword)
}

#[test]
fn vfmsub132ps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PS, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 93, 190, 154, 204], OperandSize::Dword)
}

#[test]
fn vfmsub132ps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PS, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM5)), operand3: Some(Indirect(EDX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 85, 206, 154, 26], OperandSize::Dword)
}

#[test]
fn vfmsub132ps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PS, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledIndexed(ESI, EAX, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 125, 219, 154, 20, 134], OperandSize::Dword)
}

#[test]
fn vfmsub132ps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PS, operand1: Some(Direct(ZMM25)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM23)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 34, 109, 189, 154, 207], OperandSize::Qword)
}

#[test]
fn vfmsub132ps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PS, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM11)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RDX, Four, 86912130, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 37, 205, 154, 140, 146, 130, 44, 46, 5], OperandSize::Qword)
}

#[test]
fn vfmsub132ps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PS, operand1: Some(Direct(ZMM27)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectDisplaced(RAX, 1062657244, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 98, 85, 223, 154, 152, 220, 220, 86, 63], OperandSize::Qword)
}

