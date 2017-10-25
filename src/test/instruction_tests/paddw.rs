use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn paddw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDW, operand1: Some(Direct(MM1)), operand2: Some(Direct(MM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 253, 204], OperandSize::Dword)
}

fn paddw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDW, operand1: Some(Direct(MM6)), operand2: Some(IndirectScaledDisplaced(EBX, Eight, 132665710, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 253, 52, 221, 110, 81, 232, 7], OperandSize::Dword)
}

fn paddw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDW, operand1: Some(Direct(MM5)), operand2: Some(Direct(MM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 253, 235], OperandSize::Qword)
}

fn paddw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDW, operand1: Some(Direct(MM7)), operand2: Some(IndirectScaledIndexed(RDX, RDI, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 253, 60, 186], OperandSize::Qword)
}

fn paddw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 253, 239], OperandSize::Dword)
}

fn paddw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDW, operand1: Some(Direct(XMM2)), operand2: Some(Indirect(EAX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 253, 16], OperandSize::Dword)
}

fn paddw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 253, 231], OperandSize::Qword)
}

fn paddw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDW, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexed(RDI, RDX, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 253, 20, 151], OperandSize::Qword)
}

