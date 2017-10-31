use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpabsb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSB, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 28, 220], OperandSize::Dword)
}

#[test]
fn vpabsb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSB, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledDisplaced(EBX, Four, 1488573712, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 28, 52, 157, 16, 213, 185, 88], OperandSize::Dword)
}

#[test]
fn vpabsb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 28, 241], OperandSize::Qword)
}

#[test]
fn vpabsb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSB, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexed(RAX, RDI, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 28, 52, 184], OperandSize::Qword)
}

#[test]
fn vpabsb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSB, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 28, 241], OperandSize::Dword)
}

#[test]
fn vpabsb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSB, operand1: Some(Direct(YMM2)), operand2: Some(IndirectScaledIndexed(ECX, EBX, Four, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 28, 20, 153], OperandSize::Dword)
}

#[test]
fn vpabsb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSB, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 28, 200], OperandSize::Qword)
}

#[test]
fn vpabsb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSB, operand1: Some(Direct(YMM0)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RSI, Four, 393763961, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 28, 132, 178, 121, 92, 120, 23], OperandSize::Qword)
}

#[test]
fn vpabsb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 137, 28, 242], OperandSize::Dword)
}

#[test]
fn vpabsb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSB, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, ECX, Two, 1966088999, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 140, 28, 148, 72, 39, 35, 48, 117], OperandSize::Dword)
}

#[test]
fn vpabsb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSB, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 226, 125, 137, 28, 204], OperandSize::Qword)
}

#[test]
fn vpabsb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSB, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexed(RCX, RDX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 141, 28, 52, 81], OperandSize::Qword)
}

#[test]
fn vpabsb_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSB, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 170, 28, 225], OperandSize::Dword)
}

#[test]
fn vpabsb_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSB, operand1: Some(Direct(YMM1)), operand2: Some(IndirectScaledIndexed(EDX, EDI, Four, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 175, 28, 12, 186], OperandSize::Dword)
}

#[test]
fn vpabsb_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSB, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 174, 28, 204], OperandSize::Qword)
}

#[test]
fn vpabsb_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSB, operand1: Some(Direct(YMM16)), operand2: Some(IndirectScaledIndexed(RAX, RDI, Two, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 226, 125, 170, 28, 4, 120], OperandSize::Qword)
}

#[test]
fn vpabsb_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSB, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 204, 28, 237], OperandSize::Dword)
}

#[test]
fn vpabsb_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSB, operand1: Some(Direct(ZMM1)), operand2: Some(Indirect(ECX, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 207, 28, 9], OperandSize::Dword)
}

#[test]
fn vpabsb_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSB, operand1: Some(Direct(ZMM26)), operand2: Some(Direct(ZMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 98, 125, 206, 28, 214], OperandSize::Qword)
}

#[test]
fn vpabsb_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSB, operand1: Some(Direct(ZMM16)), operand2: Some(Indirect(RBX, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 226, 125, 205, 28, 3], OperandSize::Qword)
}

