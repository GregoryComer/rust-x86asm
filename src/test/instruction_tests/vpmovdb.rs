use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpmovdb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVDB, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 126, 139, 49, 226], OperandSize::Dword)
}

fn vpmovdb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVDB, operand1: Some(IndirectScaledDisplaced(ECX, Eight, 846223130, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 8, 49, 20, 205, 26, 87, 112, 50], OperandSize::Dword)
}

fn vpmovdb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVDB, operand1: Some(Direct(XMM18)), operand2: Some(Direct(XMM26)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 34, 126, 141, 49, 210], OperandSize::Qword)
}

fn vpmovdb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVDB, operand1: Some(IndirectScaledDisplaced(RDX, Two, 53085072, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM17)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 226, 126, 8, 49, 12, 85, 144, 3, 42, 3], OperandSize::Qword)
}

fn vpmovdb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVDB, operand1: Some(Direct(XMM7)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 126, 175, 49, 247], OperandSize::Dword)
}

fn vpmovdb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVDB, operand1: Some(IndirectScaledIndexedDisplaced(ECX, EDX, Two, 1538246610, Some(OperandSize::Qword), None)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 40, 49, 180, 81, 210, 199, 175, 91], OperandSize::Dword)
}

fn vpmovdb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVDB, operand1: Some(Direct(XMM13)), operand2: Some(Direct(YMM23)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 194, 126, 169, 49, 253], OperandSize::Qword)
}

fn vpmovdb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVDB, operand1: Some(Indirect(RCX, Some(OperandSize::Qword), None)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 40, 49, 57], OperandSize::Qword)
}

fn vpmovdb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVDB, operand1: Some(Direct(XMM2)), operand2: Some(Direct(ZMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 126, 202, 49, 234], OperandSize::Dword)
}

fn vpmovdb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVDB, operand1: Some(IndirectDisplaced(ESI, 754117982, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(ZMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 49, 134, 94, 237, 242, 44], OperandSize::Dword)
}

fn vpmovdb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVDB, operand1: Some(Direct(XMM22)), operand2: Some(Direct(ZMM21)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 162, 126, 206, 49, 238], OperandSize::Qword)
}

fn vpmovdb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVDB, operand1: Some(IndirectDisplaced(RSI, 1120746213, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(ZMM13)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 114, 126, 72, 49, 174, 229, 58, 205, 66], OperandSize::Qword)
}

