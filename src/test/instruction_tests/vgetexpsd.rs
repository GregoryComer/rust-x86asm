use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vgetexpsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPSD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 213, 159, 67, 206], OperandSize::Dword)
}

fn vgetexpsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPSD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(EBX, 10141215, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 229, 143, 67, 163, 31, 190, 154, 0], OperandSize::Dword)
}

fn vgetexpsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPSD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM17)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K1), broadcast: None }, &[98, 178, 197, 153, 67, 217], OperandSize::Qword)
}

fn vgetexpsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPSD, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM15)), operand3: Some(Indirect(RCX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 114, 133, 141, 67, 57], OperandSize::Qword)
}

