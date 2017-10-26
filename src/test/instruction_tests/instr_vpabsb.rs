use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpabsb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSB, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 28, 201], OperandSize::Dword)
}

#[test]
fn vpabsb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSB, operand1: Some(Direct(XMM1)), operand2: Some(IndirectDisplaced(EAX, 836749650, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 28, 136, 82, 201, 223, 49], OperandSize::Dword)
}

#[test]
fn vpabsb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 28, 198], OperandSize::Qword)
}

#[test]
fn vpabsb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSB, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RCX, Four, 186526609, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 28, 180, 139, 145, 43, 30, 11], OperandSize::Qword)
}

#[test]
fn vpabsb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSB, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 28, 225], OperandSize::Dword)
}

#[test]
fn vpabsb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSB, operand1: Some(Direct(YMM4)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, EDX, Eight, 873869491, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 28, 164, 210, 179, 48, 22, 52], OperandSize::Dword)
}

#[test]
fn vpabsb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSB, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 28, 207], OperandSize::Qword)
}

#[test]
fn vpabsb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSB, operand1: Some(Direct(YMM0)), operand2: Some(IndirectDisplaced(RDX, 2112908564, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 28, 130, 20, 109, 240, 125], OperandSize::Qword)
}

#[test]
fn vpabsb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 141, 28, 242], OperandSize::Dword)
}

#[test]
fn vpabsb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSB, operand1: Some(Direct(XMM1)), operand2: Some(IndirectDisplaced(EAX, 1283201289, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 140, 28, 136, 9, 25, 124, 76], OperandSize::Dword)
}

#[test]
fn vpabsb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSB, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM9)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 194, 125, 143, 28, 193], OperandSize::Qword)
}

#[test]
fn vpabsb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSB, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexed(RDX, RSI, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 137, 28, 52, 178], OperandSize::Qword)
}

#[test]
fn vpabsb_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSB, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 173, 28, 228], OperandSize::Dword)
}

#[test]
fn vpabsb_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSB, operand1: Some(Direct(YMM5)), operand2: Some(Indirect(ECX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 175, 28, 41], OperandSize::Dword)
}

#[test]
fn vpabsb_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSB, operand1: Some(Direct(YMM22)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 226, 125, 169, 28, 242], OperandSize::Qword)
}

#[test]
fn vpabsb_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSB, operand1: Some(Direct(YMM15)), operand2: Some(IndirectScaledIndexed(RDX, RBX, Two, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 114, 125, 174, 28, 60, 90], OperandSize::Qword)
}

#[test]
fn vpabsb_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSB, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 204, 28, 244], OperandSize::Dword)
}

#[test]
fn vpabsb_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSB, operand1: Some(Direct(ZMM7)), operand2: Some(IndirectScaledDisplaced(EAX, Eight, 133485836, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 205, 28, 60, 197, 12, 213, 244, 7], OperandSize::Dword)
}

#[test]
fn vpabsb_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSB, operand1: Some(Direct(ZMM11)), operand2: Some(Direct(ZMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 114, 125, 201, 28, 220], OperandSize::Qword)
}

#[test]
fn vpabsb_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSB, operand1: Some(Direct(ZMM17)), operand2: Some(Indirect(RBX, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 226, 125, 203, 28, 11], OperandSize::Qword)
}

