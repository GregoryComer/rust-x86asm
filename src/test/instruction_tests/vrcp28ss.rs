use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vrcp28ss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP28SS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 77, 153, 203, 234], OperandSize::Dword)
}

fn vrcp28ss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP28SS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EBX, Two, 1044382860, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 138, 203, 188, 88, 140, 4, 64, 62], OperandSize::Dword)
}

fn vrcp28ss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP28SS, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM16)), operand3: Some(Direct(XMM26)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K1), broadcast: None }, &[98, 18, 125, 145, 203, 250], OperandSize::Qword)
}

fn vrcp28ss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCP28SS, operand1: Some(Direct(XMM31)), operand2: Some(Direct(XMM1)), operand3: Some(Indirect(RDI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 98, 117, 137, 203, 63], OperandSize::Qword)
}

