use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpmovdw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVDW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 126, 138, 51, 197], OperandSize::Dword)
}

fn vpmovdw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVDW, operand1: Some(IndirectScaledDisplaced(EAX, Four, 1289270077, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 8, 51, 12, 133, 61, 179, 216, 76], OperandSize::Dword)
}

fn vpmovdw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVDW, operand1: Some(Direct(XMM22)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 178, 126, 140, 51, 198], OperandSize::Qword)
}

fn vpmovdw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVDW, operand1: Some(IndirectScaledIndexed(RAX, RSI, Eight, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM12)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 114, 126, 8, 51, 36, 240], OperandSize::Qword)
}

fn vpmovdw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVDW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 126, 170, 51, 245], OperandSize::Dword)
}

fn vpmovdw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVDW, operand1: Some(IndirectDisplaced(EAX, 865858094, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 40, 51, 144, 46, 242, 155, 51], OperandSize::Dword)
}

fn vpmovdw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVDW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(YMM31)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 98, 126, 174, 51, 248], OperandSize::Qword)
}

fn vpmovdw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVDW, operand1: Some(IndirectScaledIndexedDisplaced(RAX, RAX, Eight, 1414659132, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 40, 51, 172, 192, 60, 252, 81, 84], OperandSize::Qword)
}

fn vpmovdw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVDW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(ZMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 126, 207, 51, 194], OperandSize::Dword)
}

fn vpmovdw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVDW, operand1: Some(Indirect(EAX, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(ZMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 51, 8], OperandSize::Dword)
}

fn vpmovdw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVDW, operand1: Some(Direct(YMM16)), operand2: Some(Direct(ZMM20)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 162, 126, 204, 51, 224], OperandSize::Qword)
}

fn vpmovdw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVDW, operand1: Some(IndirectDisplaced(RCX, 326229102, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(ZMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 51, 177, 110, 220, 113, 19], OperandSize::Qword)
}

