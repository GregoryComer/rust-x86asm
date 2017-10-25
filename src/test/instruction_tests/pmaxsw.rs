use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn pmaxsw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXSW, operand1: Some(Direct(MM2)), operand2: Some(Direct(MM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 238, 209], OperandSize::Dword)
}

fn pmaxsw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXSW, operand1: Some(Direct(MM6)), operand2: Some(IndirectScaledIndexed(EDX, ESI, Two, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 238, 52, 114], OperandSize::Dword)
}

fn pmaxsw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXSW, operand1: Some(Direct(MM6)), operand2: Some(Direct(MM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 238, 243], OperandSize::Qword)
}

fn pmaxsw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXSW, operand1: Some(Direct(MM0)), operand2: Some(IndirectScaledIndexed(RDI, RDX, Two, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 238, 4, 87], OperandSize::Qword)
}

fn pmaxsw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXSW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 238, 197], OperandSize::Dword)
}

fn pmaxsw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXSW, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, ESI, Four, 1856399531, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 238, 188, 177, 171, 104, 166, 110], OperandSize::Dword)
}

fn pmaxsw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXSW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 238, 245], OperandSize::Qword)
}

fn pmaxsw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXSW, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RCX, Two, 294245053, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 238, 156, 78, 189, 210, 137, 17], OperandSize::Qword)
}

