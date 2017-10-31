use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpaddusb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 220, 196], OperandSize::Dword)
}

#[test]
fn vpaddusb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, ECX, Eight, 123701677, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 220, 132, 202, 173, 137, 95, 7], OperandSize::Dword)
}

#[test]
fn vpaddusb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSB, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 220, 207], OperandSize::Qword)
}

#[test]
fn vpaddusb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSB, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: Some(Indirect(RDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 220, 18], OperandSize::Qword)
}

#[test]
fn vpaddusb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSB, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 220, 240], OperandSize::Dword)
}

#[test]
fn vpaddusb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSB, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledDisplaced(ECX, Eight, 823824532, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 220, 52, 205, 148, 144, 26, 49], OperandSize::Dword)
}

#[test]
fn vpaddusb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSB, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 220, 222], OperandSize::Qword)
}

#[test]
fn vpaddusb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSB, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectDisplaced(RDI, 503312072, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 220, 151, 200, 238, 255, 29], OperandSize::Qword)
}

#[test]
fn vpaddusb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSB, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 93, 141, 220, 218], OperandSize::Dword)
}

#[test]
fn vpaddusb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(EBX, EAX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 93, 137, 220, 52, 195], OperandSize::Dword)
}

#[test]
fn vpaddusb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSB, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM23)), operand3: Some(Direct(XMM27)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 129, 69, 132, 220, 251], OperandSize::Qword)
}

#[test]
fn vpaddusb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSB, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectDisplaced(RSI, 663133497, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 225, 85, 142, 220, 142, 57, 157, 134, 39], OperandSize::Qword)
}

#[test]
fn vpaddusb_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSB, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 93, 170, 220, 243], OperandSize::Dword)
}

#[test]
fn vpaddusb_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSB, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledDisplaced(ECX, Eight, 1642594290, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 85, 170, 220, 60, 205, 242, 255, 231, 97], OperandSize::Dword)
}

#[test]
fn vpaddusb_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSB, operand1: Some(Direct(YMM26)), operand2: Some(Direct(YMM11)), operand3: Some(Direct(YMM19)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 33, 37, 174, 220, 211], OperandSize::Qword)
}

#[test]
fn vpaddusb_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSB, operand1: Some(Direct(YMM27)), operand2: Some(Direct(YMM9)), operand3: Some(Indirect(RDX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 97, 53, 175, 220, 26], OperandSize::Qword)
}

#[test]
fn vpaddusb_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSB, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 93, 202, 220, 250], OperandSize::Dword)
}

#[test]
fn vpaddusb_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSB, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM0)), operand3: Some(Indirect(EDX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 125, 203, 220, 58], OperandSize::Dword)
}

#[test]
fn vpaddusb_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSB, operand1: Some(Direct(ZMM30)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM23)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 33, 69, 203, 220, 247], OperandSize::Qword)
}

#[test]
fn vpaddusb_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDUSB, operand1: Some(Direct(ZMM25)), operand2: Some(Direct(ZMM19)), operand3: Some(IndirectScaledDisplaced(RDI, Eight, 828205042, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 97, 101, 198, 220, 12, 253, 242, 103, 93, 49], OperandSize::Qword)
}

