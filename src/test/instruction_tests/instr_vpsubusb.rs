use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpsubusb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSB, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 216, 238], OperandSize::Dword)
}

#[test]
fn vpsubusb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSB, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 216, 14], OperandSize::Dword)
}

#[test]
fn vpsubusb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSB, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 216, 202], OperandSize::Qword)
}

#[test]
fn vpsubusb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSB, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: Some(Indirect(RBX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 216, 35], OperandSize::Qword)
}

#[test]
fn vpsubusb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSB, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 216, 203], OperandSize::Dword)
}

#[test]
fn vpsubusb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSB, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexed(ESI, EDI, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 216, 36, 126], OperandSize::Dword)
}

#[test]
fn vpsubusb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSB, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 216, 204], OperandSize::Qword)
}

#[test]
fn vpsubusb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSB, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectDisplaced(RBX, 2146804369, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 216, 131, 145, 162, 245, 127], OperandSize::Qword)
}

#[test]
fn vpsubusb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 101, 139, 216, 245], OperandSize::Dword)
}

#[test]
fn vpsubusb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, ECX, Four, 1203704777, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 85, 137, 216, 180, 136, 201, 19, 191, 71], OperandSize::Dword)
}

#[test]
fn vpsubusb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSB, operand1: Some(Direct(XMM12)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM8)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 81, 69, 138, 216, 224], OperandSize::Qword)
}

#[test]
fn vpsubusb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSB, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM1)), operand3: Some(Indirect(RDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 225, 117, 143, 216, 10], OperandSize::Qword)
}

#[test]
fn vpsubusb_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSB, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 109, 169, 216, 218], OperandSize::Dword)
}

#[test]
fn vpsubusb_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSB, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectDisplaced(ECX, 2061308849, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 101, 175, 216, 161, 177, 19, 221, 122], OperandSize::Dword)
}

#[test]
fn vpsubusb_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSB, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM17)), operand3: Some(Direct(YMM10)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 209, 117, 164, 216, 218], OperandSize::Qword)
}

#[test]
fn vpsubusb_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSB, operand1: Some(Direct(YMM12)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectDisplaced(RSI, 231429742, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 113, 117, 169, 216, 166, 110, 86, 203, 13], OperandSize::Qword)
}

#[test]
fn vpsubusb_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSB, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 77, 205, 216, 247], OperandSize::Dword)
}

#[test]
fn vpsubusb_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSB, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledDisplaced(EDI, Four, 1516100427, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 77, 203, 216, 12, 189, 75, 219, 93, 90], OperandSize::Dword)
}

#[test]
fn vpsubusb_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSB, operand1: Some(Direct(ZMM26)), operand2: Some(Direct(ZMM30)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 97, 13, 199, 216, 214], OperandSize::Qword)
}

#[test]
fn vpsubusb_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSUBUSB, operand1: Some(Direct(ZMM14)), operand2: Some(Direct(ZMM15)), operand3: Some(IndirectScaledDisplaced(RBX, Four, 2097189521, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 113, 5, 204, 216, 52, 157, 145, 146, 0, 125], OperandSize::Qword)
}

