use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmaddsub213ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 166, 236], OperandSize::Dword)
}

#[test]
fn vfmaddsub213ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, ESI, Eight, 176093372, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 166, 172, 247, 188, 248, 126, 10], OperandSize::Dword)
}

#[test]
fn vfmaddsub213ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 166, 216], OperandSize::Qword)
}

#[test]
fn vfmaddsub213ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(RCX, 579742412, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 166, 145, 204, 42, 142, 34], OperandSize::Qword)
}

#[test]
fn vfmaddsub213ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 166, 216], OperandSize::Dword)
}

#[test]
fn vfmaddsub213ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM7)), operand3: Some(Indirect(EDI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 166, 15], OperandSize::Dword)
}

#[test]
fn vfmaddsub213ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 166, 198], OperandSize::Qword)
}

#[test]
fn vfmaddsub213ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledDisplaced(RAX, Four, 161674657, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 166, 52, 133, 161, 245, 162, 9], OperandSize::Qword)
}

#[test]
fn vfmaddsub213ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 117, 141, 166, 237], OperandSize::Dword)
}

#[test]
fn vfmaddsub213ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: Some(Indirect(ECX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 77, 138, 166, 33], OperandSize::Dword)
}

#[test]
fn vfmaddsub213ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, ESI, Four, 1600229422, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 109, 158, 166, 188, 182, 46, 144, 97, 95], OperandSize::Dword)
}

#[test]
fn vfmaddsub213ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PS, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM26)), operand3: Some(Direct(XMM28)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 130, 45, 130, 166, 196], OperandSize::Qword)
}

#[test]
fn vfmaddsub213ps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PS, operand1: Some(Direct(XMM19)), operand2: Some(Direct(XMM29)), operand3: Some(IndirectScaledDisplaced(RAX, Two, 1654861274, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 226, 21, 129, 166, 28, 69, 218, 45, 163, 98], OperandSize::Qword)
}

#[test]
fn vfmaddsub213ps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PS, operand1: Some(Direct(XMM24)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexed(RSI, RBX, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 98, 77, 153, 166, 4, 158], OperandSize::Qword)
}

#[test]
fn vfmaddsub213ps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 109, 169, 166, 193], OperandSize::Dword)
}

#[test]
fn vfmaddsub213ps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EDI, Eight, 1706704857, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 93, 171, 166, 180, 255, 217, 63, 186, 101], OperandSize::Dword)
}

#[test]
fn vfmaddsub213ps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, ECX, Eight, 707611050, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 93, 189, 166, 148, 202, 170, 73, 45, 42], OperandSize::Dword)
}

#[test]
fn vfmaddsub213ps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PS, operand1: Some(Direct(YMM28)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM25)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 2, 117, 171, 166, 225], OperandSize::Qword)
}

#[test]
fn vfmaddsub213ps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PS, operand1: Some(Direct(YMM30)), operand2: Some(Direct(YMM30)), operand3: Some(IndirectDisplaced(RSI, 1875540290, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 98, 13, 167, 166, 182, 66, 121, 202, 111], OperandSize::Qword)
}

#[test]
fn vfmaddsub213ps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM29)), operand3: Some(IndirectDisplaced(RDX, 1484033355, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 21, 178, 166, 130, 75, 141, 116, 88], OperandSize::Qword)
}

#[test]
fn vfmaddsub213ps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PS, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 93, 221, 166, 204], OperandSize::Dword)
}

#[test]
fn vfmaddsub213ps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PS, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM5)), operand3: Some(Indirect(EDI, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 85, 203, 166, 63], OperandSize::Dword)
}

#[test]
fn vfmaddsub213ps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PS, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledIndexed(ECX, ESI, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 69, 223, 166, 52, 177], OperandSize::Dword)
}

#[test]
fn vfmaddsub213ps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PS, operand1: Some(Direct(ZMM19)), operand2: Some(Direct(ZMM11)), operand3: Some(Direct(ZMM12)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 194, 37, 157, 166, 220], OperandSize::Qword)
}

#[test]
fn vfmaddsub213ps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PS, operand1: Some(Direct(ZMM30)), operand2: Some(Direct(ZMM12)), operand3: Some(IndirectDisplaced(RCX, 1249721984, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 98, 29, 207, 166, 177, 128, 62, 125, 74], OperandSize::Qword)
}

#[test]
fn vfmaddsub213ps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PS, operand1: Some(Direct(ZMM11)), operand2: Some(Direct(ZMM31)), operand3: Some(Indirect(RSI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 114, 5, 214, 166, 30], OperandSize::Qword)
}

