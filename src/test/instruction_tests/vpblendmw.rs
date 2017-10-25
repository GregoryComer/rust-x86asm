use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpblendmw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 213, 142, 102, 230], OperandSize::Dword)
}

fn vpblendmw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledDisplaced(ESI, Four, 1644975481, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 245, 143, 102, 4, 181, 121, 85, 12, 98], OperandSize::Dword)
}

fn vpblendmw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMW, operand1: Some(Direct(XMM21)), operand2: Some(Direct(XMM25)), operand3: Some(Direct(XMM22)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 162, 181, 132, 102, 238], OperandSize::Qword)
}

fn vpblendmw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMW, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM13)), operand3: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 226, 149, 141, 102, 15], OperandSize::Qword)
}

fn vpblendmw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 221, 175, 102, 206], OperandSize::Dword)
}

fn vpblendmw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM5)), operand3: Some(Indirect(EBX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 213, 171, 102, 51], OperandSize::Dword)
}

fn vpblendmw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMW, operand1: Some(Direct(YMM29)), operand2: Some(Direct(YMM18)), operand3: Some(Direct(YMM31)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 2, 237, 164, 102, 239], OperandSize::Qword)
}

fn vpblendmw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMW, operand1: Some(Direct(YMM15)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledDisplaced(RDX, Two, 324417578, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 114, 237, 174, 102, 60, 85, 42, 56, 86, 19], OperandSize::Qword)
}

fn vpblendmw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMW, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 237, 207, 102, 209], OperandSize::Dword)
}

fn vpblendmw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMW, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledIndexed(EAX, EAX, Eight, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 253, 207, 102, 28, 192], OperandSize::Dword)
}

fn vpblendmw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMW, operand1: Some(Direct(ZMM24)), operand2: Some(Direct(ZMM10)), operand3: Some(Direct(ZMM21)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 34, 173, 205, 102, 197], OperandSize::Qword)
}

fn vpblendmw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMW, operand1: Some(Direct(ZMM10)), operand2: Some(Direct(ZMM17)), operand3: Some(IndirectScaledIndexed(RSI, RDI, Two, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 114, 245, 195, 102, 20, 126], OperandSize::Qword)
}

