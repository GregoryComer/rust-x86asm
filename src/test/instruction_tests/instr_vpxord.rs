use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpxord_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 125, 142, 239, 248], OperandSize::Dword)
}

#[test]
fn vpxord_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexed(EDX, EAX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 109, 143, 239, 4, 194], OperandSize::Dword)
}

#[test]
fn vpxord_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(EBX, Four, 1481859077, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 93, 159, 239, 60, 157, 5, 96, 83, 88], OperandSize::Dword)
}

#[test]
fn vpxord_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORD, operand1: Some(Direct(XMM25)), operand2: Some(Direct(XMM16)), operand3: Some(Direct(XMM15)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 65, 125, 132, 239, 207], OperandSize::Qword)
}

#[test]
fn vpxord_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORD, operand1: Some(Direct(XMM30)), operand2: Some(Direct(XMM28)), operand3: Some(IndirectScaledIndexed(RSI, RDX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 97, 29, 129, 239, 52, 214], OperandSize::Qword)
}

#[test]
fn vpxord_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORD, operand1: Some(Direct(XMM19)), operand2: Some(Direct(XMM21)), operand3: Some(Indirect(RAX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 225, 85, 147, 239, 24], OperandSize::Qword)
}

#[test]
fn vpxord_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 93, 172, 239, 230], OperandSize::Dword)
}

#[test]
fn vpxord_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexed(EDX, EDI, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 69, 174, 239, 44, 250], OperandSize::Dword)
}

#[test]
fn vpxord_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM3)), operand3: Some(Indirect(EDI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 101, 185, 239, 15], OperandSize::Dword)
}

#[test]
fn vpxord_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORD, operand1: Some(Direct(YMM22)), operand2: Some(Direct(YMM10)), operand3: Some(Direct(YMM20)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 161, 45, 169, 239, 244], OperandSize::Qword)
}

#[test]
fn vpxord_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM8)), operand3: Some(IndirectScaledIndexed(RDX, RAX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 61, 171, 239, 60, 66], OperandSize::Qword)
}

#[test]
fn vpxord_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORD, operand1: Some(Direct(YMM16)), operand2: Some(Direct(YMM9)), operand3: Some(Indirect(RDI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 225, 53, 190, 239, 7], OperandSize::Qword)
}

#[test]
fn vpxord_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 77, 203, 239, 196], OperandSize::Dword)
}

#[test]
fn vpxord_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM3)), operand3: Some(Indirect(ECX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 101, 205, 239, 33], OperandSize::Dword)
}

#[test]
fn vpxord_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM5)), operand3: Some(Indirect(ESI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 85, 217, 239, 38], OperandSize::Dword)
}

#[test]
fn vpxord_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORD, operand1: Some(Direct(ZMM15)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM22)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 49, 85, 202, 239, 254], OperandSize::Qword)
}

#[test]
fn vpxord_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORD, operand1: Some(Direct(ZMM27)), operand2: Some(Direct(ZMM15)), operand3: Some(IndirectDisplaced(RBX, 28012018, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 97, 5, 202, 239, 155, 242, 109, 171, 1], OperandSize::Qword)
}

#[test]
fn vpxord_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPXORD, operand1: Some(Direct(ZMM10)), operand2: Some(Direct(ZMM20)), operand3: Some(Indirect(RAX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 113, 93, 209, 239, 16], OperandSize::Qword)
}

