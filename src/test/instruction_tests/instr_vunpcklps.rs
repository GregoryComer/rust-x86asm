use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vunpcklps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 200, 20, 223], OperandSize::Dword)
}

#[test]
fn vunpcklps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(EDX, Four, 287934642, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 224, 20, 20, 149, 178, 136, 41, 17], OperandSize::Dword)
}

#[test]
fn vunpcklps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 208, 20, 233], OperandSize::Qword)
}

#[test]
fn vunpcklps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectDisplaced(RBX, 199989846, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 200, 20, 139, 86, 154, 235, 11], OperandSize::Qword)
}

#[test]
fn vunpcklps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 196, 20, 201], OperandSize::Dword)
}

#[test]
fn vunpcklps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledDisplaced(EDX, Four, 1334660482, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 196, 20, 12, 149, 130, 77, 141, 79], OperandSize::Dword)
}

#[test]
fn vunpcklps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 244, 20, 251], OperandSize::Qword)
}

#[test]
fn vunpcklps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledDisplaced(RAX, Four, 1187442707, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 228, 20, 20, 133, 19, 240, 198, 70], OperandSize::Qword)
}

#[test]
fn vunpcklps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 92, 142, 20, 229], OperandSize::Dword)
}

#[test]
fn vunpcklps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledDisplaced(EDX, Two, 1794199835, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 116, 141, 20, 28, 85, 27, 81, 241, 106], OperandSize::Dword)
}

#[test]
fn vunpcklps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, ESI, Four, 919377370, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 68, 157, 20, 172, 177, 218, 149, 204, 54], OperandSize::Dword)
}

#[test]
fn vunpcklps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPS, operand1: Some(Direct(XMM13)), operand2: Some(Direct(XMM24)), operand3: Some(Direct(XMM10)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 81, 60, 129, 20, 234], OperandSize::Qword)
}

#[test]
fn vunpcklps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPS, operand1: Some(Direct(XMM25)), operand2: Some(Direct(XMM28)), operand3: Some(Indirect(RDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 97, 28, 130, 20, 10], OperandSize::Qword)
}

#[test]
fn vunpcklps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPS, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(RSI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 225, 68, 154, 20, 14], OperandSize::Qword)
}

#[test]
fn vunpcklps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 100, 172, 20, 215], OperandSize::Dword)
}

#[test]
fn vunpcklps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledDisplaced(EAX, Eight, 2019487988, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 76, 172, 20, 36, 197, 244, 240, 94, 120], OperandSize::Dword)
}

#[test]
fn vunpcklps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EAX, Eight, 1913179528, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 108, 188, 20, 140, 193, 136, 205, 8, 114], OperandSize::Dword)
}

#[test]
fn vunpcklps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM22)), operand3: Some(Direct(YMM26)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 145, 76, 162, 20, 242], OperandSize::Qword)
}

#[test]
fn vunpcklps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexed(RSI, RDI, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 84, 169, 20, 12, 254], OperandSize::Qword)
}

#[test]
fn vunpcklps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM23)), operand3: Some(IndirectScaledDisplaced(RDI, Eight, 1115663736, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 68, 179, 20, 36, 253, 120, 173, 127, 66], OperandSize::Qword)
}

#[test]
fn vunpcklps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPS, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 100, 207, 20, 212], OperandSize::Dword)
}

#[test]
fn vunpcklps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPS, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EDI, Eight, 1373601473, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 116, 207, 20, 180, 255, 193, 126, 223, 81], OperandSize::Dword)
}

#[test]
fn vunpcklps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPS, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledDisplaced(EDX, Eight, 1984700275, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 68, 222, 20, 60, 213, 115, 31, 76, 118], OperandSize::Dword)
}

#[test]
fn vunpcklps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPS, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM8)), operand3: Some(Direct(ZMM9)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 209, 60, 204, 20, 209], OperandSize::Qword)
}

#[test]
fn vunpcklps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPS, operand1: Some(Direct(ZMM13)), operand2: Some(Direct(ZMM25)), operand3: Some(IndirectScaledDisplaced(RDI, Eight, 220613978, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 113, 52, 193, 20, 44, 253, 90, 77, 38, 13], OperandSize::Qword)
}

#[test]
fn vunpcklps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKLPS, operand1: Some(Direct(ZMM11)), operand2: Some(Direct(ZMM23)), operand3: Some(IndirectScaledDisplaced(RBX, Eight, 2141246602, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 113, 68, 213, 20, 28, 221, 138, 212, 160, 127], OperandSize::Qword)
}

