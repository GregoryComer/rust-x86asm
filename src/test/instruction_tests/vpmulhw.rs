use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpmulhw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 229, 254], OperandSize::Dword)
}

fn vpmulhw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EDX, Two, 1362152290, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 229, 140, 83, 98, 203, 48, 81], OperandSize::Dword)
}

fn vpmulhw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 229, 242], OperandSize::Qword)
}

fn vpmulhw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(RCX, RAX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 229, 20, 65], OperandSize::Qword)
}

fn vpmulhw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 229, 204], OperandSize::Dword)
}

fn vpmulhw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHW, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM6)), operand3: Some(Indirect(ESI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 229, 30], OperandSize::Dword)
}

fn vpmulhw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 229, 193], OperandSize::Qword)
}

fn vpmulhw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM7)), operand3: Some(Indirect(RSI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 229, 14], OperandSize::Qword)
}

fn vpmulhw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 109, 138, 229, 218], OperandSize::Dword)
}

fn vpmulhw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexed(ESI, EBX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 101, 143, 229, 4, 222], OperandSize::Dword)
}

fn vpmulhw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHW, operand1: Some(Direct(XMM20)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 225, 69, 141, 229, 229], OperandSize::Qword)
}

fn vpmulhw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM14)), operand3: Some(IndirectScaledDisplaced(RCX, Eight, 532933922, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 13, 141, 229, 36, 205, 34, 237, 195, 31], OperandSize::Qword)
}

fn vpmulhw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 101, 173, 229, 192], OperandSize::Dword)
}

fn vpmulhw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM5)), operand3: Some(Indirect(EAX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 85, 173, 229, 32], OperandSize::Dword)
}

fn vpmulhw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHW, operand1: Some(Direct(YMM16)), operand2: Some(Direct(YMM10)), operand3: Some(Direct(YMM9)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 193, 45, 174, 229, 193], OperandSize::Qword)
}

fn vpmulhw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM7)), operand3: Some(Indirect(RBX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 69, 175, 229, 43], OperandSize::Qword)
}

fn vpmulhw_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHW, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 109, 205, 229, 200], OperandSize::Dword)
}

fn vpmulhw_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHW, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EDX, Two, 64509818, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 109, 207, 229, 172, 86, 122, 87, 216, 3], OperandSize::Dword)
}

fn vpmulhw_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHW, operand1: Some(Direct(ZMM30)), operand2: Some(Direct(ZMM16)), operand3: Some(Direct(ZMM19)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 33, 125, 195, 229, 243], OperandSize::Qword)
}

fn vpmulhw_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHW, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM31)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RDX, Two, 1709561421, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 5, 197, 229, 180, 82, 77, 214, 229, 101], OperandSize::Qword)
}

