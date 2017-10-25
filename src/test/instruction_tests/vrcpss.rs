use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vrcpss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCPSS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 234, 83, 254], OperandSize::Dword)
}

fn vrcpss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCPSS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledDisplaced(EAX, Two, 1713941616, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 194, 83, 52, 69, 112, 172, 40, 102], OperandSize::Dword)
}

fn vrcpss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCPSS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 234, 83, 220], OperandSize::Qword)
}

fn vrcpss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRCPSS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(Indirect(RDX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 242, 83, 2], OperandSize::Qword)
}

