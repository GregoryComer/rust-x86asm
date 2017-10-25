use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpmovqb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQB, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 126, 140, 50, 220], OperandSize::Dword)
}

fn vpmovqb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQB, operand1: Some(Indirect(ESI, Some(OperandSize::Word), None)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 8, 50, 62], OperandSize::Dword)
}

fn vpmovqb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQB, operand1: Some(Direct(XMM26)), operand2: Some(Direct(XMM22)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 130, 126, 141, 50, 242], OperandSize::Qword)
}

fn vpmovqb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQB, operand1: Some(Indirect(RSI, Some(OperandSize::Word), None)), operand2: Some(Direct(XMM15)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 114, 126, 8, 50, 62], OperandSize::Qword)
}

fn vpmovqb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQB, operand1: Some(Direct(XMM7)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 126, 173, 50, 255], OperandSize::Dword)
}

fn vpmovqb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQB, operand1: Some(IndirectScaledDisplaced(EBX, Two, 1887987965, Some(OperandSize::Dword), None)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 40, 50, 20, 93, 253, 104, 136, 112], OperandSize::Dword)
}

fn vpmovqb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQB, operand1: Some(Direct(XMM11)), operand2: Some(Direct(YMM13)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 82, 126, 175, 50, 235], OperandSize::Qword)
}

fn vpmovqb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQB, operand1: Some(IndirectScaledIndexedDisplaced(RSI, RBX, Two, 627547697, Some(OperandSize::Dword), None)), operand2: Some(Direct(YMM8)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 114, 126, 40, 50, 132, 94, 49, 158, 103, 37], OperandSize::Qword)
}

fn vpmovqb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(ZMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 126, 207, 50, 254], OperandSize::Dword)
}

fn vpmovqb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQB, operand1: Some(Indirect(EAX, Some(OperandSize::Qword), None)), operand2: Some(Direct(ZMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 50, 24], OperandSize::Dword)
}

fn vpmovqb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQB, operand1: Some(Direct(XMM17)), operand2: Some(Direct(ZMM23)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 162, 126, 206, 50, 249], OperandSize::Qword)
}

fn vpmovqb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQB, operand1: Some(Indirect(RBX, Some(OperandSize::Qword), None)), operand2: Some(Direct(ZMM12)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 114, 126, 72, 50, 35], OperandSize::Qword)
}

