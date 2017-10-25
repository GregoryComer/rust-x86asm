use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vcomisd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMISD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 47, 195], OperandSize::Dword)
}

fn vcomisd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMISD, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, EAX, Four, 1268945025, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 47, 156, 129, 129, 144, 162, 75], OperandSize::Dword)
}

fn vcomisd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMISD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 47, 224], OperandSize::Qword)
}

fn vcomisd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMISD, operand1: Some(Direct(XMM2)), operand2: Some(Indirect(RDX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 47, 18], OperandSize::Qword)
}

fn vcomisd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMISD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: None, broadcast: None }, &[98, 241, 253, 24, 47, 245], OperandSize::Dword)
}

fn vcomisd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMISD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(ECX, Eight, 357913194, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 47, 36, 205, 106, 82, 85, 21], OperandSize::Dword)
}

fn vcomisd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMISD, operand1: Some(Direct(XMM26)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: None, broadcast: None }, &[98, 97, 253, 24, 47, 211], OperandSize::Qword)
}

fn vcomisd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMISD, operand1: Some(Direct(XMM10)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RSI, Four, 438989646, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 121, 47, 148, 178, 78, 115, 42, 26], OperandSize::Qword)
}

