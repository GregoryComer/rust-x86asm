use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfnmsub231ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 190, 199], OperandSize::Dword)
}

#[test]
fn vfnmsub231ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexed(ESI, EAX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 190, 60, 134], OperandSize::Dword)
}

#[test]
fn vfnmsub231ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 190, 199], OperandSize::Qword)
}

#[test]
fn vfnmsub231ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(RAX, 1098401574, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 190, 128, 38, 71, 120, 65], OperandSize::Qword)
}

#[test]
fn vfnmsub231ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 190, 239], OperandSize::Dword)
}

#[test]
fn vfnmsub231ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexed(EDI, ECX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 190, 44, 207], OperandSize::Dword)
}

#[test]
fn vfnmsub231ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 190, 243], OperandSize::Qword)
}

#[test]
fn vfnmsub231ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledDisplaced(RAX, Eight, 1235191551, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 190, 52, 197, 255, 134, 159, 73], OperandSize::Qword)
}

#[test]
fn vfnmsub231ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 77, 142, 190, 224], OperandSize::Dword)
}

#[test]
fn vfnmsub231ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledDisplaced(ECX, Four, 1232443686, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 77, 142, 190, 20, 141, 38, 153, 117, 73], OperandSize::Dword)
}

#[test]
fn vfnmsub231ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexed(EBX, EBX, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 109, 153, 190, 12, 155], OperandSize::Dword)
}

#[test]
fn vfnmsub231ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PS, operand1: Some(Direct(XMM8)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 114, 77, 137, 190, 196], OperandSize::Qword)
}

#[test]
fn vfnmsub231ps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM23)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RSI, Two, 21824481, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 69, 133, 190, 172, 118, 225, 3, 77, 1], OperandSize::Qword)
}

#[test]
fn vfnmsub231ps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PS, operand1: Some(Direct(XMM20)), operand2: Some(Direct(XMM16)), operand3: Some(IndirectScaledIndexed(RDI, RAX, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 226, 125, 145, 190, 36, 135], OperandSize::Qword)
}

#[test]
fn vfnmsub231ps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 93, 175, 190, 228], OperandSize::Dword)
}

#[test]
fn vfnmsub231ps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectDisplaced(EDI, 2103936590, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 170, 190, 159, 78, 134, 103, 125], OperandSize::Dword)
}

#[test]
fn vfnmsub231ps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EBX, Two, 611091719, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 85, 191, 190, 156, 88, 7, 133, 108, 36], OperandSize::Dword)
}

#[test]
fn vfnmsub231ps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM8)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 210, 69, 173, 190, 208], OperandSize::Qword)
}

#[test]
fn vfnmsub231ps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PS, operand1: Some(Direct(YMM17)), operand2: Some(Direct(YMM11)), operand3: Some(IndirectScaledDisplaced(RAX, Eight, 762668319, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 226, 37, 169, 190, 12, 197, 31, 101, 117, 45], OperandSize::Qword)
}

#[test]
fn vfnmsub231ps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PS, operand1: Some(Direct(YMM30)), operand2: Some(Direct(YMM22)), operand3: Some(IndirectDisplaced(RBX, 1176938789, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 98, 77, 182, 190, 179, 37, 169, 38, 70], OperandSize::Qword)
}

#[test]
fn vfnmsub231ps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PS, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM3)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 93, 190, 190, 219], OperandSize::Dword)
}

#[test]
fn vfnmsub231ps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PS, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EDI, Four, 109421133, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 101, 202, 190, 140, 187, 77, 162, 133, 6], OperandSize::Dword)
}

#[test]
fn vfnmsub231ps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PS, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EBX, Four, 1096576217, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 77, 222, 190, 132, 155, 217, 108, 92, 65], OperandSize::Dword)
}

#[test]
fn vfnmsub231ps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PS, operand1: Some(Direct(ZMM20)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM19)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 162, 125, 249, 190, 227], OperandSize::Qword)
}

#[test]
fn vfnmsub231ps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PS, operand1: Some(Direct(ZMM14)), operand2: Some(Direct(ZMM6)), operand3: Some(Indirect(RBX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 114, 77, 206, 190, 51], OperandSize::Qword)
}

#[test]
fn vfnmsub231ps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PS, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM16)), operand3: Some(IndirectScaledIndexed(RDI, RCX, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 125, 209, 190, 20, 79], OperandSize::Qword)
}

