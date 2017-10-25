use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vexpandps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXPANDPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 143, 136, 253], OperandSize::Dword)
}

fn vexpandps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXPANDPS, operand1: Some(Direct(XMM5)), operand2: Some(IndirectDisplaced(ESI, 367391600, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 143, 136, 174, 112, 243, 229, 21], OperandSize::Dword)
}

fn vexpandps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXPANDPS, operand1: Some(Direct(XMM29)), operand2: Some(Direct(XMM20)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 34, 125, 142, 136, 236], OperandSize::Qword)
}

fn vexpandps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXPANDPS, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexed(RDI, RSI, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 138, 136, 28, 247], OperandSize::Qword)
}

fn vexpandps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXPANDPS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 174, 136, 222], OperandSize::Dword)
}

fn vexpandps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXPANDPS, operand1: Some(Direct(YMM1)), operand2: Some(Indirect(EBX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 173, 136, 11], OperandSize::Dword)
}

fn vexpandps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXPANDPS, operand1: Some(Direct(YMM13)), operand2: Some(Direct(YMM8)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 82, 125, 172, 136, 232], OperandSize::Qword)
}

fn vexpandps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXPANDPS, operand1: Some(Direct(YMM27)), operand2: Some(IndirectDisplaced(RDX, 2080945680, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 98, 125, 173, 136, 154, 16, 182, 8, 124], OperandSize::Qword)
}

fn vexpandps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXPANDPS, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 206, 136, 201], OperandSize::Dword)
}

fn vexpandps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXPANDPS, operand1: Some(Direct(ZMM1)), operand2: Some(Indirect(EDX, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 201, 136, 10], OperandSize::Dword)
}

fn vexpandps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXPANDPS, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM20)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 178, 125, 206, 136, 244], OperandSize::Qword)
}

fn vexpandps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXPANDPS, operand1: Some(Direct(ZMM29)), operand2: Some(Indirect(RDX, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 98, 125, 205, 136, 42], OperandSize::Qword)
}

