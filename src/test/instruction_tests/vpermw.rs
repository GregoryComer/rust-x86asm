use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpermw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 213, 137, 141, 204], OperandSize::Dword)
}

fn vpermw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectDisplaced(EAX, 17380590, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 213, 137, 141, 152, 238, 52, 9, 1], OperandSize::Dword)
}

fn vpermw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMW, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM13)), operand3: Some(Direct(XMM31)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 130, 149, 142, 141, 199], OperandSize::Qword)
}

fn vpermw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMW, operand1: Some(Direct(XMM9)), operand2: Some(Direct(XMM13)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RSI, Four, 126679003, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 114, 149, 140, 141, 140, 177, 219, 247, 140, 7], OperandSize::Qword)
}

fn vpermw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMW, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 245, 171, 141, 253], OperandSize::Dword)
}

fn vpermw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMW, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledDisplaced(EDX, Four, 602063992, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 205, 171, 141, 60, 149, 120, 196, 226, 35], OperandSize::Dword)
}

fn vpermw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM13)), operand3: Some(Direct(YMM27)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 146, 149, 169, 141, 203], OperandSize::Qword)
}

fn vpermw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM17)), operand3: Some(IndirectDisplaced(RDI, 806038486, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 245, 164, 141, 167, 214, 43, 11, 48], OperandSize::Qword)
}

fn vpermw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMW, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 213, 207, 141, 227], OperandSize::Dword)
}

fn vpermw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMW, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledIndexed(EDX, ESI, Eight, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 237, 204, 141, 4, 242], OperandSize::Dword)
}

fn vpermw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMW, operand1: Some(Direct(ZMM23)), operand2: Some(Direct(ZMM26)), operand3: Some(Direct(ZMM10)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 194, 173, 197, 141, 250], OperandSize::Qword)
}

fn vpermw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMW, operand1: Some(Direct(ZMM28)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledIndexed(RCX, RDX, Four, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 98, 245, 207, 141, 36, 145], OperandSize::Qword)
}

