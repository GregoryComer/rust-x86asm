use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpmovzxbw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 48, 194], OperandSize::Dword)
}

fn vpmovzxbw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBW, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexed(EAX, EDX, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 48, 28, 144], OperandSize::Dword)
}

fn vpmovzxbw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 48, 233], OperandSize::Qword)
}

fn vpmovzxbw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBW, operand1: Some(Direct(XMM7)), operand2: Some(IndirectDisplaced(RBX, 1488005651, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 48, 187, 19, 42, 177, 88], OperandSize::Qword)
}

fn vpmovzxbw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBW, operand1: Some(Direct(YMM3)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 48, 216], OperandSize::Dword)
}

fn vpmovzxbw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBW, operand1: Some(Direct(YMM0)), operand2: Some(IndirectScaledIndexed(EAX, EBX, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 48, 4, 152], OperandSize::Dword)
}

fn vpmovzxbw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 48, 206], OperandSize::Qword)
}

fn vpmovzxbw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBW, operand1: Some(Direct(YMM2)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RCX, Eight, 998959087, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 48, 148, 206, 239, 231, 138, 59], OperandSize::Qword)
}

fn vpmovzxbw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 139, 48, 225], OperandSize::Dword)
}

fn vpmovzxbw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBW, operand1: Some(Direct(XMM6)), operand2: Some(IndirectDisplaced(EBX, 767732622, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 140, 48, 179, 142, 171, 194, 45], OperandSize::Dword)
}

fn vpmovzxbw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBW, operand1: Some(Direct(XMM31)), operand2: Some(Direct(XMM28)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 2, 125, 143, 48, 252], OperandSize::Qword)
}

fn vpmovzxbw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBW, operand1: Some(Direct(XMM15)), operand2: Some(IndirectScaledIndexed(RDX, RAX, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 114, 125, 138, 48, 60, 130], OperandSize::Qword)
}

fn vpmovzxbw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 173, 48, 238], OperandSize::Dword)
}

fn vpmovzxbw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBW, operand1: Some(Direct(YMM4)), operand2: Some(Indirect(EAX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 171, 48, 32], OperandSize::Dword)
}

fn vpmovzxbw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBW, operand1: Some(Direct(YMM18)), operand2: Some(Direct(XMM12)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 194, 125, 170, 48, 212], OperandSize::Qword)
}

fn vpmovzxbw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBW, operand1: Some(Direct(YMM24)), operand2: Some(IndirectScaledIndexed(RAX, RDI, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 98, 125, 171, 48, 4, 184], OperandSize::Qword)
}

fn vpmovzxbw_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBW, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 204, 48, 194], OperandSize::Dword)
}

fn vpmovzxbw_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBW, operand1: Some(Direct(ZMM4)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, ESI, Eight, 1030630525, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 206, 48, 164, 246, 125, 44, 110, 61], OperandSize::Dword)
}

fn vpmovzxbw_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBW, operand1: Some(Direct(ZMM12)), operand2: Some(Direct(YMM14)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 82, 125, 207, 48, 230], OperandSize::Qword)
}

fn vpmovzxbw_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXBW, operand1: Some(Direct(ZMM10)), operand2: Some(IndirectScaledDisplaced(RDX, Four, 365841241, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 114, 125, 207, 48, 20, 149, 89, 75, 206, 21], OperandSize::Qword)
}

