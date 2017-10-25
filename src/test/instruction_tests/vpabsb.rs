use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpabsb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSB, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 28, 218], OperandSize::Dword)
}

fn vpabsb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSB, operand1: Some(Direct(XMM1)), operand2: Some(IndirectDisplaced(ECX, 2029998837, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 28, 137, 245, 82, 255, 120], OperandSize::Dword)
}

fn vpabsb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 28, 193], OperandSize::Qword)
}

fn vpabsb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSB, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RDI, Two, 1597728404, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 28, 156, 127, 148, 102, 59, 95], OperandSize::Qword)
}

fn vpabsb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSB, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 28, 236], OperandSize::Dword)
}

fn vpabsb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSB, operand1: Some(Direct(YMM3)), operand2: Some(IndirectScaledDisplaced(ESI, Four, 2061640993, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 28, 28, 181, 33, 37, 226, 122], OperandSize::Dword)
}

fn vpabsb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSB, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 28, 235], OperandSize::Qword)
}

fn vpabsb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSB, operand1: Some(Direct(YMM7)), operand2: Some(IndirectDisplaced(RDX, 1874531318, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 28, 186, 246, 19, 187, 111], OperandSize::Qword)
}

fn vpabsb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 139, 28, 196], OperandSize::Dword)
}

fn vpabsb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSB, operand1: Some(Direct(XMM7)), operand2: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 138, 28, 62], OperandSize::Dword)
}

fn vpabsb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSB, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 141, 28, 202], OperandSize::Qword)
}

fn vpabsb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSB, operand1: Some(Direct(XMM1)), operand2: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 138, 28, 14], OperandSize::Qword)
}

fn vpabsb_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSB, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 172, 28, 203], OperandSize::Dword)
}

fn vpabsb_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSB, operand1: Some(Direct(YMM7)), operand2: Some(IndirectScaledDisplaced(EAX, Four, 890768866, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 173, 28, 60, 133, 226, 13, 24, 53], OperandSize::Dword)
}

fn vpabsb_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSB, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM18)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 178, 125, 174, 28, 194], OperandSize::Qword)
}

fn vpabsb_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSB, operand1: Some(Direct(YMM31)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RCX, Eight, 1158897628, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 98, 125, 170, 28, 188, 206, 220, 95, 19, 69], OperandSize::Qword)
}

fn vpabsb_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSB, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 207, 28, 205], OperandSize::Dword)
}

fn vpabsb_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSB, operand1: Some(Direct(ZMM1)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, EAX, Two, 866809860, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 203, 28, 140, 71, 4, 120, 170, 51], OperandSize::Dword)
}

fn vpabsb_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSB, operand1: Some(Direct(ZMM24)), operand2: Some(Direct(ZMM20)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 34, 125, 205, 28, 196], OperandSize::Qword)
}

fn vpabsb_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPABSB, operand1: Some(Direct(ZMM20)), operand2: Some(Indirect(RCX, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 226, 125, 202, 28, 33], OperandSize::Qword)
}

