use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpmaddwd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDWD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 245, 201], OperandSize::Dword)
}

fn vpmaddwd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDWD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexed(EDI, ECX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 245, 20, 143], OperandSize::Dword)
}

fn vpmaddwd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDWD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 245, 254], OperandSize::Qword)
}

fn vpmaddwd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDWD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(RAX, Four, 593357031, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 245, 12, 133, 231, 232, 93, 35], OperandSize::Qword)
}

fn vpmaddwd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDWD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 245, 206], OperandSize::Dword)
}

fn vpmaddwd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDWD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM7)), operand3: Some(Indirect(ESI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 245, 46], OperandSize::Dword)
}

fn vpmaddwd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDWD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 245, 230], OperandSize::Qword)
}

fn vpmaddwd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDWD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledDisplaced(RSI, Two, 1727881367, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 245, 28, 117, 151, 96, 253, 102], OperandSize::Qword)
}

fn vpmaddwd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDWD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 117, 140, 245, 222], OperandSize::Dword)
}

fn vpmaddwd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDWD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(ECX, Four, 1077896082, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 85, 137, 245, 20, 141, 146, 99, 63, 64], OperandSize::Dword)
}

fn vpmaddwd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDWD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM14)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 209, 93, 141, 245, 222], OperandSize::Qword)
}

fn vpmaddwd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDWD, operand1: Some(Direct(XMM26)), operand2: Some(Direct(XMM23)), operand3: Some(IndirectDisplaced(RAX, 893933617, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 97, 69, 134, 245, 144, 49, 88, 72, 53], OperandSize::Qword)
}

fn vpmaddwd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDWD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 109, 169, 245, 233], OperandSize::Dword)
}

fn vpmaddwd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDWD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexed(EAX, ESI, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 85, 173, 245, 12, 176], OperandSize::Dword)
}

fn vpmaddwd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDWD, operand1: Some(Direct(YMM19)), operand2: Some(Direct(YMM10)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 225, 45, 170, 245, 223], OperandSize::Qword)
}

fn vpmaddwd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDWD, operand1: Some(Direct(YMM24)), operand2: Some(Direct(YMM11)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RCX, Four, 1315364780, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 97, 37, 174, 245, 132, 136, 172, 223, 102, 78], OperandSize::Qword)
}

fn vpmaddwd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDWD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 93, 202, 245, 208], OperandSize::Dword)
}

fn vpmaddwd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDWD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledDisplaced(EBX, Four, 462283691, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 93, 201, 245, 44, 157, 171, 227, 141, 27], OperandSize::Dword)
}

fn vpmaddwd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDWD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM8)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 209, 93, 204, 245, 200], OperandSize::Qword)
}

fn vpmaddwd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDWD, operand1: Some(Direct(ZMM11)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RSI, Four, 1003478040, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 113, 77, 205, 245, 156, 179, 24, 220, 207, 59], OperandSize::Qword)
}

