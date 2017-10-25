use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vrsqrt28ss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT28SS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 101, 159, 205, 217], OperandSize::Dword)
}

fn vrsqrt28ss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT28SS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(Indirect(EBX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 85, 139, 205, 43], OperandSize::Dword)
}

fn vrsqrt28ss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT28SS, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM25)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K6), broadcast: None }, &[98, 226, 53, 150, 205, 248], OperandSize::Qword)
}

fn vrsqrt28ss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT28SS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM25)), operand3: Some(IndirectScaledDisplaced(RDX, Eight, 496021011, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 53, 131, 205, 36, 213, 19, 174, 144, 29], OperandSize::Qword)
}

