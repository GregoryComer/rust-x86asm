use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmsub231ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 186, 246], OperandSize::Dword)
}

#[test]
fn vfmsub231ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(EAX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 186, 48], OperandSize::Dword)
}

#[test]
fn vfmsub231ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 186, 198], OperandSize::Qword)
}

#[test]
fn vfmsub231ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 186, 46], OperandSize::Qword)
}

#[test]
fn vfmsub231ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 186, 208], OperandSize::Dword)
}

#[test]
fn vfmsub231ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexed(EDI, EDI, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 93, 186, 20, 127], OperandSize::Dword)
}

#[test]
fn vfmsub231ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 186, 217], OperandSize::Qword)
}

#[test]
fn vfmsub231ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM6)), operand3: Some(Indirect(RDX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 77, 186, 10], OperandSize::Qword)
}

#[test]
fn vfmsub231ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 109, 142, 186, 246], OperandSize::Dword)
}

#[test]
fn vfmsub231ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexed(EBX, EAX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 109, 140, 186, 44, 195], OperandSize::Dword)
}

#[test]
fn vfmsub231ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(EDX, 1395663649, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 117, 155, 186, 130, 33, 35, 48, 83], OperandSize::Dword)
}

#[test]
fn vfmsub231ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PS, operand1: Some(Direct(XMM30)), operand2: Some(Direct(XMM9)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 98, 53, 137, 186, 242], OperandSize::Qword)
}

#[test]
fn vfmsub231ps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PS, operand1: Some(Direct(XMM31)), operand2: Some(Direct(XMM23)), operand3: Some(IndirectDisplaced(RDI, 272751508, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 98, 69, 130, 186, 191, 148, 219, 65, 16], OperandSize::Qword)
}

#[test]
fn vfmsub231ps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PS, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM28)), operand3: Some(Indirect(RAX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 114, 29, 146, 186, 16], OperandSize::Qword)
}

#[test]
fn vfmsub231ps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 85, 169, 186, 207], OperandSize::Dword)
}

#[test]
fn vfmsub231ps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledDisplaced(EBX, Four, 200059215, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 85, 171, 186, 60, 157, 79, 169, 236, 11], OperandSize::Dword)
}

#[test]
fn vfmsub231ps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexed(ESI, ECX, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 85, 188, 186, 60, 206], OperandSize::Dword)
}

#[test]
fn vfmsub231ps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PS, operand1: Some(Direct(YMM10)), operand2: Some(Direct(YMM11)), operand3: Some(Direct(YMM16)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 50, 37, 174, 186, 208], OperandSize::Qword)
}

#[test]
fn vfmsub231ps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PS, operand1: Some(Direct(YMM21)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RSI, Two, 329698621, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 226, 109, 170, 186, 172, 119, 61, 205, 166, 19], OperandSize::Qword)
}

#[test]
fn vfmsub231ps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PS, operand1: Some(Direct(YMM21)), operand2: Some(Direct(YMM2)), operand3: Some(Indirect(RCX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 226, 109, 190, 186, 41], OperandSize::Qword)
}

#[test]
fn vfmsub231ps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PS, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 109, 222, 186, 236], OperandSize::Dword)
}

#[test]
fn vfmsub231ps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PS, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectDisplaced(EAX, 61306965, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 93, 204, 186, 168, 85, 120, 167, 3], OperandSize::Dword)
}

#[test]
fn vfmsub231ps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PS, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledIndexed(EDI, EAX, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 125, 221, 186, 4, 135], OperandSize::Dword)
}

#[test]
fn vfmsub231ps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PS, operand1: Some(Direct(ZMM24)), operand2: Some(Direct(ZMM16)), operand3: Some(Direct(ZMM28)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 2, 125, 180, 186, 196], OperandSize::Qword)
}

#[test]
fn vfmsub231ps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PS, operand1: Some(Direct(ZMM15)), operand2: Some(Direct(ZMM18)), operand3: Some(IndirectDisplaced(RDI, 162702876, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 114, 109, 195, 186, 191, 28, 166, 178, 9], OperandSize::Qword)
}

#[test]
fn vfmsub231ps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PS, operand1: Some(Direct(ZMM13)), operand2: Some(Direct(ZMM13)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RBX, Two, 882093335, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 114, 21, 220, 186, 172, 89, 23, 173, 147, 52], OperandSize::Qword)
}

