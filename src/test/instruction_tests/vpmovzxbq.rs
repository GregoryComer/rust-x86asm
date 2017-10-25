use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpmovzxbq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 50, 223], OperandSize::Dword)
}

fn vpmovzxbq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBQ, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledDisplaced(ESI, Four, 1894398505, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 50, 20, 181, 41, 58, 234, 112], OperandSize::Dword)
}

fn vpmovzxbq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 50, 205], OperandSize::Qword)
}

fn vpmovzxbq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBQ, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledDisplaced(RDX, Eight, 800561621, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 50, 12, 213, 213, 153, 183, 47], OperandSize::Qword)
}

fn vpmovzxbq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBQ, operand1: Some(Direct(YMM7)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 50, 255], OperandSize::Dword)
}

fn vpmovzxbq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBQ, operand1: Some(Direct(YMM5)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, EDX, Four, 203520538, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 50, 172, 144, 26, 122, 33, 12], OperandSize::Dword)
}

fn vpmovzxbq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 50, 224], OperandSize::Qword)
}

fn vpmovzxbq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBQ, operand1: Some(Direct(YMM5)), operand2: Some(IndirectScaledIndexed(RCX, RSI, Two, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 50, 44, 113], OperandSize::Qword)
}

fn vpmovzxbq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 143, 50, 235], OperandSize::Dword)
}

fn vpmovzxbq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBQ, operand1: Some(Direct(XMM2)), operand2: Some(Indirect(EAX, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 141, 50, 16], OperandSize::Dword)
}

fn vpmovzxbq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBQ, operand1: Some(Direct(XMM12)), operand2: Some(Direct(XMM17)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 50, 125, 138, 50, 225], OperandSize::Qword)
}

fn vpmovzxbq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBQ, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RSI, Eight, 1086755928, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 143, 50, 132, 247, 88, 148, 198, 64], OperandSize::Qword)
}

fn vpmovzxbq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 172, 50, 240], OperandSize::Dword)
}

fn vpmovzxbq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBQ, operand1: Some(Direct(YMM3)), operand2: Some(IndirectDisplaced(EDX, 978566235, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 173, 50, 154, 91, 188, 83, 58], OperandSize::Dword)
}

fn vpmovzxbq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBQ, operand1: Some(Direct(YMM24)), operand2: Some(Direct(XMM14)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 66, 125, 173, 50, 198], OperandSize::Qword)
}

fn vpmovzxbq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBQ, operand1: Some(Direct(YMM31)), operand2: Some(IndirectScaledIndexed(RBX, RCX, Four, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 98, 125, 175, 50, 60, 139], OperandSize::Qword)
}

fn vpmovzxbq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBQ, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 202, 50, 219], OperandSize::Dword)
}

fn vpmovzxbq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBQ, operand1: Some(Direct(ZMM3)), operand2: Some(IndirectDisplaced(EDX, 770776140, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 201, 50, 154, 76, 28, 241, 45], OperandSize::Dword)
}

fn vpmovzxbq_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBQ, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(XMM17)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 178, 125, 202, 50, 233], OperandSize::Qword)
}

fn vpmovzxbq_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBQ, operand1: Some(Direct(ZMM23)), operand2: Some(Indirect(RAX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 226, 125, 204, 50, 56], OperandSize::Qword)
}

