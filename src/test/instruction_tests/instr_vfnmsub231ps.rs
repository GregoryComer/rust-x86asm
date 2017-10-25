use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfnmsub231ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 190, 223], OperandSize::Dword)
}

#[test]
fn vfnmsub231ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(EDX, EBX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 190, 44, 90], OperandSize::Dword)
}

#[test]
fn vfnmsub231ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 190, 205], OperandSize::Qword)
}

#[test]
fn vfnmsub231ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(RDX, RAX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 190, 28, 194], OperandSize::Qword)
}

#[test]
fn vfnmsub231ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 190, 217], OperandSize::Dword)
}

#[test]
fn vfnmsub231ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexed(EBX, ECX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 77, 190, 28, 203], OperandSize::Dword)
}

#[test]
fn vfnmsub231ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 190, 248], OperandSize::Qword)
}

#[test]
fn vfnmsub231ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM3)), operand3: Some(Indirect(RDI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 190, 39], OperandSize::Qword)
}

#[test]
fn vfnmsub231ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 77, 142, 190, 227], OperandSize::Dword)
}

#[test]
fn vfnmsub231ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(ESI, 695858586, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 117, 141, 190, 166, 154, 245, 121, 41], OperandSize::Dword)
}

#[test]
fn vfnmsub231ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(ESI, EDI, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 93, 158, 190, 28, 126], OperandSize::Dword)
}

#[test]
fn vfnmsub231ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PS, operand1: Some(Direct(XMM20)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM12)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 194, 85, 138, 190, 228], OperandSize::Qword)
}

#[test]
fn vfnmsub231ps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM31)), operand3: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 5, 133, 190, 54], OperandSize::Qword)
}

#[test]
fn vfnmsub231ps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PS, operand1: Some(Direct(XMM31)), operand2: Some(Direct(XMM24)), operand3: Some(IndirectScaledIndexed(RDI, RDX, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 98, 61, 147, 190, 60, 87], OperandSize::Qword)
}

#[test]
fn vfnmsub231ps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 117, 169, 190, 199], OperandSize::Dword)
}

#[test]
fn vfnmsub231ps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, ECX, Eight, 1406704760, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 101, 173, 190, 164, 206, 120, 156, 216, 83], OperandSize::Dword)
}

#[test]
fn vfnmsub231ps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectDisplaced(ESI, 1287325240, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 109, 185, 190, 182, 56, 6, 187, 76], OperandSize::Dword)
}

#[test]
fn vfnmsub231ps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM31)), operand3: Some(Direct(YMM24)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 146, 5, 165, 190, 232], OperandSize::Qword)
}

#[test]
fn vfnmsub231ps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PS, operand1: Some(Direct(YMM31)), operand2: Some(Direct(YMM12)), operand3: Some(IndirectDisplaced(RAX, 690448747, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 98, 29, 175, 190, 184, 107, 105, 39, 41], OperandSize::Qword)
}

#[test]
fn vfnmsub231ps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM18)), operand3: Some(Indirect(RSI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 109, 182, 190, 14], OperandSize::Qword)
}

#[test]
fn vfnmsub231ps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PS, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 93, 253, 190, 214], OperandSize::Dword)
}

#[test]
fn vfnmsub231ps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PS, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledDisplaced(EAX, Eight, 1807882323, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 101, 205, 190, 44, 197, 83, 24, 194, 107], OperandSize::Dword)
}

#[test]
fn vfnmsub231ps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PS, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectDisplaced(EDI, 1060855824, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 93, 221, 190, 151, 16, 96, 59, 63], OperandSize::Dword)
}

#[test]
fn vfnmsub231ps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PS, operand1: Some(Direct(ZMM29)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM28)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 2, 101, 185, 190, 236], OperandSize::Qword)
}

#[test]
fn vfnmsub231ps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PS, operand1: Some(Direct(ZMM23)), operand2: Some(Direct(ZMM20)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RDI, Eight, 693070257, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 226, 93, 197, 190, 188, 250, 177, 105, 79, 41], OperandSize::Qword)
}

#[test]
fn vfnmsub231ps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PS, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM30)), operand3: Some(IndirectScaledDisplaced(RDX, Four, 1279900066, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 13, 211, 190, 60, 149, 162, 185, 73, 76], OperandSize::Qword)
}

