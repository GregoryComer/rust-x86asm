use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmsub213ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 170, 234], OperandSize::Dword)
}

#[test]
fn vfmsub213ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(EDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 170, 18], OperandSize::Dword)
}

#[test]
fn vfmsub213ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 170, 207], OperandSize::Qword)
}

#[test]
fn vfmsub213ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(RDI, Eight, 1944521592, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 170, 60, 253, 120, 11, 231, 115], OperandSize::Qword)
}

#[test]
fn vfmsub213ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 77, 170, 249], OperandSize::Dword)
}

#[test]
fn vfmsub213ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectDisplaced(ECX, 674542991, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 170, 177, 143, 181, 52, 40], OperandSize::Dword)
}

#[test]
fn vfmsub213ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 170, 212], OperandSize::Qword)
}

#[test]
fn vfmsub213ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexed(RAX, RSI, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 170, 52, 176], OperandSize::Qword)
}

#[test]
fn vfmsub213ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 77, 142, 170, 231], OperandSize::Dword)
}

#[test]
fn vfmsub213ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexed(ECX, EAX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 117, 138, 170, 20, 65], OperandSize::Dword)
}

#[test]
fn vfmsub213ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(EDX, EAX, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 93, 154, 170, 44, 194], OperandSize::Dword)
}

#[test]
fn vfmsub213ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM31)), operand3: Some(Direct(XMM15)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 210, 5, 131, 170, 255], OperandSize::Qword)
}

#[test]
fn vfmsub213ps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PS, operand1: Some(Direct(XMM22)), operand2: Some(Direct(XMM31)), operand3: Some(IndirectDisplaced(RCX, 1788740291, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 226, 5, 132, 170, 177, 195, 2, 158, 106], OperandSize::Qword)
}

#[test]
fn vfmsub213ps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM25)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RDX, Eight, 587561336, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 53, 149, 170, 180, 215, 120, 121, 5, 35], OperandSize::Qword)
}

#[test]
fn vfmsub213ps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 77, 174, 170, 246], OperandSize::Dword)
}

#[test]
fn vfmsub213ps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexed(ECX, ECX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 69, 171, 170, 52, 73], OperandSize::Dword)
}

#[test]
fn vfmsub213ps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexed(EDI, ECX, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 101, 186, 170, 60, 143], OperandSize::Dword)
}

#[test]
fn vfmsub213ps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM29)), operand3: Some(Direct(YMM31)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 146, 21, 161, 170, 247], OperandSize::Qword)
}

#[test]
fn vfmsub213ps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PS, operand1: Some(Direct(YMM24)), operand2: Some(Direct(YMM31)), operand3: Some(IndirectScaledIndexed(RBX, RCX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 98, 5, 162, 170, 4, 203], OperandSize::Qword)
}

#[test]
fn vfmsub213ps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PS, operand1: Some(Direct(YMM19)), operand2: Some(Direct(YMM23)), operand3: Some(IndirectScaledDisplaced(RDX, Two, 1837858443, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 226, 69, 179, 170, 28, 85, 139, 126, 139, 109], OperandSize::Qword)
}

#[test]
fn vfmsub213ps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PS, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM2)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 93, 187, 170, 218], OperandSize::Dword)
}

#[test]
fn vfmsub213ps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PS, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EAX, Two, 1941673830, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 109, 203, 170, 188, 70, 102, 151, 187, 115], OperandSize::Dword)
}

#[test]
fn vfmsub213ps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PS, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledDisplaced(ESI, Two, 57202767, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 93, 223, 170, 36, 117, 79, 216, 104, 3], OperandSize::Dword)
}

#[test]
fn vfmsub213ps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PS, operand1: Some(Direct(ZMM19)), operand2: Some(Direct(ZMM26)), operand3: Some(Direct(ZMM10)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 194, 45, 247, 170, 218], OperandSize::Qword)
}

#[test]
fn vfmsub213ps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PS, operand1: Some(Direct(ZMM18)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledIndexed(RBX, RSI, Eight, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 226, 85, 203, 170, 20, 243], OperandSize::Qword)
}

#[test]
fn vfmsub213ps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PS, operand1: Some(Direct(ZMM9)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RBX, Two, 611104981, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 114, 109, 219, 170, 140, 90, 213, 184, 108, 36], OperandSize::Qword)
}

